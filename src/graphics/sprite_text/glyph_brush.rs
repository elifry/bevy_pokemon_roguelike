use bevy::{prelude::*, text::Text2dBounds};
use image::{ImageBuffer, RgbaImage};

use crate::graphics::assets::font_assets::FontSheet;

use super::{utils::extract_sub_image, SpriteTextSection};

#[derive(Debug, Clone)]
pub(crate) struct PositionedGlyph {
    pub glyph_id: u32,
    pub position: UVec2,
    pub image: RgbaImage,
}

pub(crate) fn calculate_glyph_positions(
    section: &SpriteTextSection,
    font_sheet: &FontSheet,
    texture_atlas: &TextureAtlas,
    texture_image: &Image,
    bounds: &Text2dBounds, // Assuming bounds is a struct that contains size and other properties
) -> (Vec<PositionedGlyph>, u32, u32) {
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
    max_height *= line_number;

    (glyphs, max_width, max_height)
}
