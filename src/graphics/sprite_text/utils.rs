use bevy::prelude::*;
use bitmap_font::bfn::BoundingBox;
use image::{ImageBuffer, Rgba, RgbaImage};

pub(crate) fn extract_sub_image(img: &Image, bbox: &BoundingBox) -> Option<RgbaImage> {
    // Validate the rectangle dimensions
    if bbox.width == 0 || bbox.height == 0 {
        return None;
    }

    // Create a new image buffer for the sub-image
    let mut sub_img: RgbaImage = ImageBuffer::new(bbox.width as u32, bbox.height as u32);

    let atlas_image_width = img.texture_descriptor.size.width;

    // Calculate the number of bytes per row (assuming RGBA format, hence * 4)
    let bytes_per_row = atlas_image_width as usize * 4;

    for y in 0..bbox.height {
        for x in 0..bbox.width {
            let pixel_index = ((bbox.y + y) * bytes_per_row) + ((bbox.x + x) * 4);

            let red = img.data[pixel_index];
            let green = img.data[pixel_index + 1];
            let blue = img.data[pixel_index + 2];
            let alpha = img.data[pixel_index + 3];

            let rgba: Rgba<u8> = Rgba([red, green, blue, alpha]);

            sub_img.put_pixel(x as u32, y as u32, rgba);
        }
    }

    Some(sub_img)
}
