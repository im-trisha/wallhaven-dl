use super::crop_and_resize;
use crate::Error;
use futures::StreamExt;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};
use wallhaven_rs::{Resolution, WallhavenClient};

pub async fn download_wallpaper(
    id: &str,
    client: &WallhavenClient,
    outdir: &PathBuf,
    resolution: Option<Resolution>,
) -> crate::Result<()> {
    log::info!("Starting process for wallpaper {id}...");

    log::debug!("Fetching information for {id}...");
    let wallpaper = client.wallpaper(&id).await?;
    let original_res = wallpaper.resolution.clone();
    log::debug!("Fetched information for {id}.");

    log::debug!("Downloading {id}...");
    let mut buffer = Vec::with_capacity(wallpaper.file_size as usize);
    let mut stream = client.download_wallpaper(&wallpaper).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| Error::FileDownloading)?;
        buffer.extend_from_slice(&chunk);
    }
    log::debug!("Downloaded {id}.");

    log::debug!("Cropping {id}...");
    let file_type = wallpaper.file_type.subtype().as_str();
    let (buffer, str_res, file_type) = if let Some(res) = resolution {
        // Run cropping on a dedicated thread to avoid blocking async runtime
        let resized = tokio::task::spawn_blocking(move || crop_and_resize(&buffer, &res)).await??;
        (resized, format!("{res}"), "png")
    } else {
        (buffer, format!("{original_res}"), file_type)
    };

    log::debug!("Cropped {id}.");

    log::debug!("Creating file for {id}...");
    let fname = format!("wallhaven-{}_{str_res}.{file_type}", wallpaper.id);

    let mut file = File::create(outdir.join(fname)).await?;
    file.write_all(&buffer).await?;
    log::debug!("File created for {id}...");

    log::info!("Finished process for wallpaper {id}.");
    Ok(())
}
