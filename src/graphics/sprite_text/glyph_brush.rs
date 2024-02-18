use bevy::prelude::*;
use bitmap_font::{
    bfn,
    fonts::{BitmapFont, BitmapFontData},
};
use image::{ImageBuffer, RgbaImage};
use unicode_linebreak::BreakOpportunity;

use super::utils::extract_sub_image;

#[derive(Debug, Clone)]
pub(crate) struct PositionedGlyph {
    pub glyph_id: u32,
    pub position: UVec2,
    pub image: RgbaImage,
}

pub struct GlyphCalculatedLayout<'a> {
    pub height: f32,
    pub width: f32,
    pub lines: Vec<Vec<GlyphLine<'a>>>,
}

pub struct GlyphLine<'a> {
    pub font: &'a bfn::Font,
    pub glyph: &'a bfn::Glyph,
    pub section_index: usize,
}

pub struct TextSection<'a> {
    pub text: &'a str,
    pub font: &'a bfn::Font,
}

pub(crate) fn process_glyph_layout<'a>(
    text_sections: &'a [TextSection<'a>],
    max_width: Option<usize>,
) -> Option<GlyphCalculatedLayout<'a>> {
    let whole_text = text_sections
        .iter()
        .map(|text_section| text_section.text)
        .collect::<Vec<_>>()
        .join("");

    // Calculate line breaks for the text
    let mut line_breaks = unicode_linebreak::linebreaks(&whole_text).collect::<Vec<_>>();
    line_breaks.reverse();
    let line_breaks = line_breaks; // Make immutable

    // Create a vector that holds all of the lines of the text and the glyphs in each line
    let mut lines: Vec<Vec<GlyphLine>> = Default::default();

    // Start glyph layout
    let mut current_line = Vec::new();
    let mut line_x = 0; // The x position in the line we are currently at

    for (section_index, text_section) in text_sections.iter().enumerate() {
        let font = text_section.font;
        let text = text_section.text;
        let default_glyph = font.glyphs.get(&(' ' as u32));

        for (char_i, char) in text.char_indices() {
            // Get the glyph for this character
            let glyph = font
                .glyphs
                .get(&(char as u32))
                .or(default_glyph)
                .unwrap_or_else(|| panic!("Font does not contain glyph for character: {:?}", char));

            // Add the next glyph to the current line
            current_line.push(GlyphLine {
                font,
                glyph: glyph,
                section_index,
            });

            // Wrap the line if necessary
            if let Some(max_width) = max_width {
                // Calculate the new x position of the line after adding this glyph
                line_x += glyph.bounds.width;

                // If this character must break the line
                if line_breaks
                 .iter()
                 .any(|(i, op)| i == &(char_i + 1) && op == &BreakOpportunity::Mandatory)
                 // The last character always breaks, but we want to ignore that one
                 && char_i != text.len() - 1
                {
                    // Add this line to the lines list
                    lines.push(current_line);
                    // Start a new line
                    current_line = Vec::new();
                    // Reset the line x position
                    line_x = 0;

                // If the new line x goes over our max width, we need to find the last position
                // we can break the line
                } else if line_x > max_width {
                    for (break_i, line_break) in &line_breaks {
                        match (break_i, line_break) {
                            // We found a spot that we can break the line
                            (split_i, unicode_linebreak::BreakOpportunity::Allowed)
                                if split_i < &char_i =>
                            {
                                // Figure out how many character will be broken off
                                let broken_chars = char_i - split_i;
                                // Get the point in the line at which to break it
                                let split_at = current_line.len() - 1 - broken_chars;
                                // Split the broken off characters into a new line
                                let next_line = current_line.split_off(split_at);
                                // Add the current line to the lines list
                                lines.push(current_line);
                                // Set the new current line to the next line
                                current_line = next_line;
                                // Reset our current line x counter to the length of the new
                                // current line
                                line_x = current_line
                                    .iter()
                                    .fold(0, |width, gl| width + gl.glyph.bounds.width);
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    // Push the last line
    lines.push(current_line);

    // Determine the size of the label
    let line_height = (text_sections[0].font.char_height) as f32; // TODO: calculate the real line height
    let height = line_height * lines.len() as f32;
    let width = lines.iter().fold(0, |width, line| {
        let line_width = line
            .iter()
            .fold(0, |width, gl| width + gl.glyph.bounds.width);

        if line_width > width {
            line_width
        } else {
            width
        }
    }) as f32;

    Some(GlyphCalculatedLayout {
        width,
        height,
        lines,
    })
}
