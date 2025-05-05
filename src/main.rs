use clap::Parser;
use error::WallpaperError;
use image_processing::crop_and_resize;
use std::path::PathBuf;
use tokio::{fs, task::JoinSet};
use wallhaven_api::WallhavenClient;

pub use error::Error;
mod args;
mod error;
mod firefox;
mod image_processing;
use firefox::models::Firefox;

const API_KEY_NAME: &str = "WALLHAVEN_API_KEY";
const WALLHAVEN_PREFIX: &str = "https://wallhaven.cc/w/";

#[tokio::main]
async fn main() -> error::Result<()> {
    let args = args::Args::parse();

    let outdir = PathBuf::from(shellexpand::full(&args.outdir)?.to_string());
    fs::create_dir_all(&outdir).await?;

    let api_key = args.api_key.or_else(|| std::env::var(API_KEY_NAME).ok());
    let client = WallhavenClient::new(api_key.as_deref())?;

    let mut firefox = Firefox::new().await?;

    let mut wallhaven_ids = Vec::new();
    for profile in &mut firefox.profiles {
        for window in &mut profile.profile.windows {
            window.tabs.retain(|t| {
                let mut urls: Vec<String> = t
                    .entries
                    .iter()
                    .filter_map(|e| e.url.strip_prefix(WALLHAVEN_PREFIX).map(String::from))
                    .collect();

                let is_empty = urls.is_empty();
                wallhaven_ids.append(&mut urls);
                is_empty
            });
        }
    }

    let mut set: JoinSet<Result<(), WallpaperError>> = JoinSet::new();

    for id in wallhaven_ids {
        let client = client.clone();
        let outdir = outdir.clone();
        let resolution = args.resolution.clone();

        set.spawn(async move {
            let wallpaper = client.wallpaper(&id).await;
            let wallpaper = wallpaper.map_err(|e| WallpaperError::from(&id, e))?;

            let path = wallpaper.download(&client, &outdir).await;
            let path = path.map_err(|e| WallpaperError::from(&id, e))?;

            let with_new_ext = path.clone().with_extension("png");

            if let Some(resolution) = resolution {
                crop_and_resize(&path, &with_new_ext, &resolution)
                    .map_err(|e| WallpaperError::from(&id, e))?;
            }

            if path != with_new_ext {
                fs::remove_file(path)
                    .await
                    .map_err(|e| WallpaperError::from(&id, e))?;
            }
            Ok(())
        });
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Err(e)) => eprintln!("{}", e),
            Err(err) => eprintln!("Error joining thread: {err}"),
            Ok(Ok(_)) => {}
        }
    }

    firefox.save().await?;

    Ok(())
}
