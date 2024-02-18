use bevy::prelude::*;
use bitmap_font::bfn::BoundingBox;
use image::{ImageBuffer, Rgba, RgbaImage};

/// Extract a sub image from the bbox coordinates into a new image
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

// Function to add color to a single pixel
pub fn add_color_to_pixel(pixel: Rgba<u8>, add_color: Rgba<u8>) -> Rgba<u8> {
    let (pr, pg, pb, pa) = (
        pixel[0] as u32,
        pixel[1] as u32,
        pixel[2] as u32,
        pixel[3] as u32,
    );
    let (ar, ag, ab, _) = (
        add_color[0] as u32,
        add_color[1] as u32,
        add_color[2] as u32,
        add_color[3] as u32,
    );

    // Calculate the added color; clamp the result to the maximum value of 255
    let add = |p: u32, a: u32| -> u8 { ((p + a).min(255)) as u8 };

    Rgba([add(pr, ar), add(pg, ag), add(pb, ab), pa as u8]) // Keep the original alpha
}

// Function to subtract color from a single pixel
pub fn subtract_color_from_pixel(pixel: Rgba<u8>, subtract_color: Rgba<u8>) -> Rgba<u8> {
    let (pr, pg, pb, pa) = (
        pixel[0] as i32,
        pixel[1] as i32,
        pixel[2] as i32,
        pixel[3] as i32,
    );
    let (sr, sg, sb, _) = (
        subtract_color[0] as i32,
        subtract_color[1] as i32,
        subtract_color[2] as i32,
        subtract_color[3] as i32,
    );

    // Calculate the subtracted color; clamp the result to the minimum value of 0
    let subtract = |p: i32, s: i32| -> u8 { ((p - s).max(0)) as u8 };

    Rgba([
        subtract(pr, sr),
        subtract(pg, sg),
        subtract(pb, sb),
        pa as u8,
    ]) // Keep the original alpha
}

pub fn blend_pixel(pixel: Rgba<u8>, blend_color: Rgba<u8>) -> Rgba<u8> {
    let (pr, pg, pb, pa) = (
        pixel[0] as f32,
        pixel[1] as f32,
        pixel[2] as f32,
        pixel[3] as f32,
    );
    let (br, bg, bb, ba) = (
        blend_color[0] as f32,
        blend_color[1] as f32,
        blend_color[2] as f32,
        blend_color[3] as f32,
    );

    // Calculate the blended color; simple alpha blending
    let alpha = ba / 255.0;
    let blend = |p: f32, b: f32| -> u8 { (((1.0 - alpha) * p) + (alpha * b)).round() as u8 };

    Rgba([blend(pr, br), blend(pg, bg), blend(pb, bb), pixel[3]]) // Keep the original alpha
}

pub fn invert_color(color: (u8, u8, u8)) -> (u8, u8, u8) {
    let (r, g, b) = color;
    (255 - r, 255 - g, 255 - b)
}
