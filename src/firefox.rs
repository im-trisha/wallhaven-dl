pub mod models;

use models::{Firefox, Profile, ProfileWithPath};
use std::ffi::OsStr;
use std::io::{self, ErrorKind, Read};
use std::path::PathBuf;
use std::str;
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;

use crate::error::Result;

const WALLHAVEN_PREFIX: &str = "https://wallhaven.cc/w/";

impl Firefox {
    pub async fn new() -> Result<Self> {
        Self::ensure_closed();
        let path = Self::get_sessionstore_path()?;
        let path = path.to_str().ok_or(crate::Error::InvalidFirefoxPath)?;

        let mut profiles = Vec::with_capacity(16);

        for result in glob::glob(path)? {
            let path = result?;
            let json = Self::decompressed_contents(&path).await?;
            let profile: Profile = serde_json::from_str(&json)?;
            profiles.push(ProfileWithPath { profile, path });
        }

        Ok(Self { profiles })
    }

    pub fn ensure_closed() {
        let mut sys = sysinfo::System::new_all();

        let process_name = OsStr::new("firefox");
        while sys.processes_by_name(process_name).next().is_some() {
            log::warn!("Close firefox then press any key...");
            let mut buf = [0; 1];
            std::io::stdin().read_exact(&mut buf).unwrap();
            sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
        }
    }

    pub async fn save(&self) -> Result<()> {
        for ProfileWithPath { profile, path } in &self.profiles {
            let json = serde_json::to_string_pretty(&profile)?;
            let bytes = json.as_bytes();
            let compressed = lz4::block::compress(bytes, None, true)?;

            let mut with_magic = "mozLz40\0".as_bytes().to_vec();
            with_magic.extend(compressed);

            fs::write(path, with_magic).await?;
        }

        Ok(())
    }

    pub fn wallhaven_urls(&self) -> Vec<String> {
        self.profiles
            .iter()
            .flat_map(|profile| &profile.profile.windows)
            .flat_map(|window| &window.tabs)
            .flat_map(|tab| &tab.entries)
            .filter_map(|entry| entry.url.strip_prefix(WALLHAVEN_PREFIX).map(String::from))
            .collect()
    }

    pub fn remove_ids<S: AsRef<str>>(&mut self, ids: &[S]) {
        let ids: Vec<&str> = ids.iter().map(|i| i.as_ref()).collect();
        for profile in &mut self.profiles {
            for window in &mut profile.profile.windows {
                window.tabs.retain(|t| {
                    t.entries.iter().any(|e| {
                        e.url
                            .strip_prefix(WALLHAVEN_PREFIX)
                            .is_some_and(|u| ids.contains(&u))
                    })
                });
            }
        }
    }

    async fn decompressed_contents(path: &PathBuf) -> Result<String> {
        let mut input_file = File::open(path).await?;
        let mut input_buffer = Vec::new();
        input_file.read_to_end(&mut input_buffer).await?;

        // Skip the first 8 bytes: "mozLz40\0"
        let blocks = lz4::block::decompress(&input_buffer[8..], None)?;
        Ok(str::from_utf8(&blocks[..])?.to_string())
    }

    fn get_sessionstore_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or(io::Error::new(
            ErrorKind::Other,
            "Couldn't find home directory",
        ))?;
        let home = home.to_str().ok_or(crate::Error::InvalidHomePath)?;

        #[cfg(target_os = "linux")]
        #[rustfmt::skip]
        let path: PathBuf = [home, ".mozilla", "firefox", "*default*", "sessionstore.jsonlz4"].iter().collect();

        #[cfg(target_os = "macos")]
        #[rustfmt::skip]
        let path: PathBuf = [home, "Library", "Application Support", "Firefox", "Profiles", "*default*", "sessionstore.jsonlz4"].iter().collect();

        #[cfg(target_os = "windows")]
        #[rustfmt::skip]
        let path: PathBuf = [home, "AppData", "Roaming", "Firefox", "Profiles", "*default*", "sessionstore.jsonlz4"].iter().collect();

        Ok(path)
    }
}
