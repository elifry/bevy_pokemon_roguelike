use bevy::{prelude::*, sprite::Anchor, text::Text2dBounds};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

use crate::graphics::assets::font_assets::FontSheet;

use super::{SpriteText, SpriteTextSection};

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

pub(crate) fn calculate_glyph_positions(
    section: &SpriteTextSection,
    font_sheet: &FontSheet,
    texture_atlas: &TextureAtlas,
    texture_image: &Image,
    bounds: &Text2dBounds, // Assuming bounds is a struct that contains size and other properties
) -> (Vec<(u32, RgbaImage, u32, u32)>, u32, u32) {
    let mut line_number = 1;
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;
    let mut dx: u32 = 0;
    let mut dy: u32 = 0;
    let mut current_line_width: u32 = 0;
    let space_width = 5; // Assuming a fixed width for spaces

    let mut glyphs = Vec::new();

    for character in section.value.chars() {
        if character == '\n' {
            // Handle explicit line break
            dx = 0;
            dy += max_height;
            max_width = max_width.max(current_line_width);
            current_line_width = 0;
            max_height = 0; // Reset max height for the next line
            line_number += 1;
            continue;
        }

        let glyph_id = character as u32;
        let (glyph_width, glyph_image) = if character == ' ' {
            (space_width, ImageBuffer::new(space_width, max_height)) // Use max_height for consistent line height
        } else if let Some(glyph) = font_sheet.glyphs.get(&glyph_id) {
            let glyph_rect = texture_atlas.textures[glyph.index];
            let glyph_image =
                extract_sub_image(texture_image, &glyph_rect).expect("Failed to extract sub-image");
            (glyph_image.width(), glyph_image)
        } else {
            warn!("couldn't find the character '{}'", character);
            (space_width, ImageBuffer::new(space_width, 0))
        };

        max_height = max_height.max(glyph_image.height());

        // Check if the current glyph exceeds the bounds and a line break is needed
        if bounds.size.x < (dx + glyph_width) as f32 {
            dx = 0; // Reset dx for the new line
            dy += max_height; // Move dy to the next line
            line_number += 1;
            max_width = max_width.max(current_line_width);
            current_line_width = 0; // Reset current line width
            max_height = glyph_image.height(); // Reset max height for the new line
        }

        glyphs.push((glyph_id, glyph_image, dx, dy));

        // Update dx and current line width
        dx += glyph_width;
        current_line_width = current_line_width.max(dx);
    }

    // After iterating through all characters, update max_width for the last line if needed
    max_width = max_width.max(current_line_width);
    max_height *= line_number;

    (glyphs, max_width, max_height)
}
