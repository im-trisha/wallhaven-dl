use clap::Parser;
use std::{collections::HashSet, path::PathBuf, sync::Arc};
use tokio::{fs, task::JoinSet};
use wallhaven_rs::WallhavenClient;

pub use error::{Error, Result};
mod args;
mod error;
mod firefox;
mod steps;
use firefox::models::Firefox;

const API_KEY_NAME: &str = "WALLHAVEN_API_KEY";

#[tokio::main]
async fn main() -> error::Result<()> {
    steps::setup_logger()?;

    let args = args::Args::parse();

    let outdir = PathBuf::from(shellexpand::full(&args.outdir)?.to_string());
    fs::create_dir_all(&outdir).await?;

    let api_key = args.api_key.or_else(|| std::env::var(API_KEY_NAME).ok());
    let client = match api_key {
        Some(key) => WallhavenClient::with_key(key)?,
        None => WallhavenClient::new()?,
    };

    let mut firefox = Firefox::new().await?;
    let mut wallhaven_ids = firefox.wallhaven_urls();

    let mut set: JoinSet<core::result::Result<(), (String, crate::Error)>> = JoinSet::new();

    let outdir = Arc::new(outdir);
    log::info!(
        "Found {} wallpapers to download. Starting...",
        wallhaven_ids.len()
    );

    for id in &wallhaven_ids {
        let client = client.clone();
        let outdir = Arc::clone(&outdir);
        let resolution = args.resolution.clone();
        let id = id.clone();

        set.spawn(async move {
            match steps::download_wallpaper(&id, &client, &outdir, resolution).await {
                Ok(_) => Ok(()),
                Err(e) => Err((id, e)),
            }
        });
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(Err((id, e))) => {
                log::error!("Error for wallpaper {id}: {e}");
                wallhaven_ids.remove(&id);
            }
            Err(e) => log::error!("Error joining thread: {e}"),
            Ok(Ok(_)) => {}
        }
    }

    if !args.no_remove {
        firefox.remove_ids(&wallhaven_ids);
        firefox.save().await?;
    }

    Ok(())
}
