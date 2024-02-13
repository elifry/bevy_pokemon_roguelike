use bevy::prelude::*;
use image::{ImageBuffer, Rgba, RgbaImage};

pub(crate) fn extract_sub_image(img: &Image, rect: &Rect) -> Option<RgbaImage> {
    let width: u32 = rect.width() as u32;
    let height: u32 = rect.height() as u32;

    // Validate the rectangle dimensions
    if width == 0
        || height == 0
        || rect.max.x as u32 > img.width()
        || rect.max.y as u32 > img.height()
    {
        return None;
    }

    // Create a new image buffer for the sub-image
    let mut sub_img: RgbaImage = ImageBuffer::new(width, height);

    let atlas_image_width = img.texture_descriptor.size.width;

    // Calculate the number of bytes per row (assuming RGBA format, hence * 4)
    let bytes_per_row = atlas_image_width as usize * 4;

    for y in 0..height {
        for x in 0..width {
            let pixel_index = ((rect.min.y + y as f32) as usize * bytes_per_row)
                + ((rect.min.x + x as f32) as usize * 4);

            let red = img.data[pixel_index];
            let green = img.data[pixel_index + 1];
            let blue = img.data[pixel_index + 2];
            let alpha = img.data[pixel_index + 3];

            let rgba: Rgba<u8> = Rgba([red, green, blue, alpha]);

            sub_img.put_pixel(x, y, rgba);
        }
    }

    Some(sub_img)
}
