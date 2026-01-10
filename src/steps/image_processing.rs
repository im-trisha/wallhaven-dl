use std::io::Cursor;

use image::imageops::FilterType;
use wallhaven_rs::Resolution;

pub fn crop_and_resize(buffer: &[u8], res: &Resolution) -> crate::Result<Vec<u8>> {
    let (target_w, target_h) = res.dimensions();

    let img = image::load_from_memory(buffer)?;
    let img = img.resize_to_fill(target_w, target_h, FilterType::Lanczos3);

    let mut out = Vec::with_capacity(1 * 1024 * 1024);
    img.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png)?;

    Ok(out)
}
