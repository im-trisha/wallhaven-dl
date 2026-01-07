use std::io::Cursor;

use image::{GenericImageView, imageops::FilterType};
use wallhaven_rs::Resolution;

pub fn crop_and_resize(buffer: &[u8], res: &Resolution) -> crate::Result<Vec<u8>> {
    let (target_w, target_h) = res.dimensions();

    let img = image::load_from_memory(buffer)?;
    let (orig_w, orig_h) = img.dimensions();

    if orig_w == target_w && orig_h == target_h {
        return Ok(buffer.to_vec());
    }

    let (crop_w, crop_h) = match orig_w * orig_h > target_w * target_h {
        true => (orig_h * target_w / target_h, orig_h),
        _ => (orig_w, orig_w * (target_h / target_w)),
    };

    let crop_x = ((orig_w as isize - crop_w as isize).abs() / 2) as u32;
    let crop_y = ((orig_h as isize - crop_h as isize).abs() / 2) as u32;

    let cropped = img.crop_imm(crop_x, crop_y, crop_w, crop_h);
    let resized = cropped.resize_exact(target_w, target_h, FilterType::Lanczos3);

    let mut out = Vec::with_capacity(1 * 1024 * 1024);
    resized.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png)?;

    Ok(out)
}
