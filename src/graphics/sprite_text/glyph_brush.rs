use bevy::prelude::*;
use image::{ImageBuffer, RgbaImage};

use crate::graphics::assets::font_assets::FontSheet;

use super::utils::extract_sub_image;

#[derive(Debug, Clone)]
pub(crate) struct PositionedGlyph {
    pub glyph_id: u32,
    pub position: UVec2,
    pub image: RgbaImage,
}

// process_glyphs
pub(crate) fn calculate_glyph_positions(
    text: &str,
    font_sheet: &FontSheet,
    texture_atlas: &TextureAtlas,
    texture_image: &Image,
    bounds: &Vec2,
    start_position: Option<UVec2>,
) -> (Vec<PositionedGlyph>, u32, u32) {
    let start_position = start_position.unwrap_or(UVec2::ZERO);

    let mut line_number = 1;
    let mut max_width: u32 = 0;

    let mut dx: u32 = start_position.x;
    let mut dy: u32 = start_position.y;

    let mut current_line_width: u32 = 0;
    let mut line_height: u32 = 0;

    let space_width = 5; // Assuming a fixed width for spaces

    let mut glyphs = Vec::new();

    for character in text.chars() {
        if character == '\n' {
            // Handle explicit line break
            dx = 0;
            dy += line_height;
            max_width = max_width.max(current_line_width);
            current_line_width = 0;
            line_height = 0; // Reset max height for the next line
            line_number += 1;
            continue;
        }

        let glyph_id = character as u32;
        let (glyph_width, glyph_image) = if character == ' ' {
            (space_width, ImageBuffer::new(space_width, line_height)) // Use max_height for consistent line height
        } else if let Some(glyph) = font_sheet.glyphs.get(&glyph_id) {
            let glyph_rect = texture_atlas.textures[glyph.index];
            let glyph_image =
                extract_sub_image(texture_image, &glyph_rect).expect("Failed to extract sub-image");
            (glyph_image.width(), glyph_image)
        } else {
            warn!("couldn't find the character '{}'", character);
            (space_width, ImageBuffer::new(space_width, 0))
        };

        line_height = line_height.max(glyph_image.height());

        // Check if the current glyph exceeds the bounds and a line break is needed
        if bounds.x < (dx + glyph_width) as f32 {
            dx = 0; // Reset dx for the new line
            dy += line_height; // Move dy to the next line
            line_number += 1;
            max_width = max_width.max(current_line_width);
            current_line_width = 0; // Reset current line width
            line_height = glyph_image.height(); // Reset max height for the new line
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

    (glyphs, max_width, line_height)
}
