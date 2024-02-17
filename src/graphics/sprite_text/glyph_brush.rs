use bevy::prelude::*;
use bitmap_font::bfn;
use image::{ImageBuffer, RgbaImage};

use super::utils::extract_sub_image;

#[derive(Debug, Clone)]
pub(crate) struct PositionedGlyph {
    pub glyph_id: u32,
    pub position: UVec2,
    pub image: RgbaImage,
}

pub(crate) fn process_glyph(
    text: &str,
    font: &bfn::Font,
    texture_image: &Image,
    bounds: &Vec2,
    start_position: Option<UVec2>,
) -> (Vec<PositionedGlyph>, u32) {
    let start_position = start_position.unwrap_or(UVec2::ZERO);

    let mut max_width: u32 = 0;

    let mut dx: u32 = start_position.x;
    let mut dy: u32 = start_position.y;

    let mut current_line_width: u32 = 0;

    let mut glyphs = Vec::new();

    for character in text.chars() {
        if character == '\n' {
            // Handle explicit line break
            dx = 0;
            dy += font.char_height;
            max_width = max_width.max(current_line_width);
            current_line_width = 0;
            continue;
        }

        let glyph_id = character as u32;
        let (glyph_width, glyph_image) = if character == ' ' {
            (
                font.space_width,
                ImageBuffer::new(font.space_width, font.char_height),
            ) // Use max_height for consistent line height
        } else if let Some(glyph) = font.glyphs.get(&glyph_id) {
            let glyph_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                extract_sub_image(texture_image, &glyph.bounds)
                    .expect("Failed to extract sub-image");
            (glyph_image.width(), glyph_image)
        } else {
            warn!("couldn't find the character '{}'", character);
            (
                font.space_width,
                ImageBuffer::new(font.space_width, font.char_height),
            )
        };

        // Check if the current glyph exceeds the bounds and a line break is needed
        if bounds.x < (dx + glyph_width) as f32 {
            dx = 0; // Reset dx for the new line
            dy += font.char_height as u32; // Move dy to the next line
            max_width = max_width.max(current_line_width);
            current_line_width = 0; // Reset current line width
        }

        glyphs.push(PositionedGlyph {
            glyph_id,
            position: UVec2::new(dx, dy),
            image: glyph_image,
        });

        // Update dx and current line width
        dx += glyph_width;
        current_line_width = current_line_width.max(dx);
    }

    // After iterating through all characters, update max_width for the last line if needed
    max_width = max_width.max(current_line_width);

    (glyphs, max_width)
}

// pub fn calculate_glyphs()
