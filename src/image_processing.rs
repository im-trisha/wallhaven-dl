use std::path::PathBuf;

use image::{GenericImageView, imageops::FilterType};
use wallhaven_api::Resolutions;

pub fn crop_and_resize(
    input_path: &PathBuf,
    output_path: &PathBuf,
    res: &Resolutions,
) -> crate::error::Result<()> {
    let (target_w, target_h) = res.dimensions();
    let target_aspect = target_w as f32 / target_h as f32;

    let img = image::open(&input_path)?;
    let (orig_w, orig_h) = img.dimensions();
    let orig_aspect = orig_w as f32 / orig_h as f32;

    let (crop_w, crop_h) = if orig_aspect > target_aspect {
        // Image is too wide
        let crop_w = (orig_h as f32 * target_aspect).round() as u32;
        (crop_w, orig_h)
    } else {
        // Image is too tall
        let crop_h = (orig_w as f32 / target_aspect).round() as u32;
        (orig_w, crop_h)
    };

    let crop_x = (orig_w - crop_w) / 2;
    let crop_y = (orig_h - crop_h) / 2;

    let cropped = img.crop_imm(crop_x, crop_y, crop_w, crop_h);
    let resized = cropped.resize_exact(target_w, target_h, FilterType::Lanczos3);

    resized.save_with_format(output_path, image::ImageFormat::Png)?;

    Ok(())
}
