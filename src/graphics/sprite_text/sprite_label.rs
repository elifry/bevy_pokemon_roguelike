//! Bitmap font label widget

use bevy::{log::info, prelude::Handle};
use bevy_egui::egui::{self, Mesh, Widget};
use bitmap_font::{bfn, fonts::BitmapFont, BitmapFontCache, BitmapFontCacheItem};
use unicode_linebreak::BreakOpportunity;

pub struct SpriteLabel<'a> {
    pub text: &'a str,
    pub font: &'a Handle<BitmapFont>,
    pub color: egui::Color32,
}

pub struct SpriteLabelCalculatedLayout {
    pub font_cache: BitmapFontCacheItem,
    pub lines: Vec<Vec<bfn::Glyph>>,
    pub line_height: f32,
    pub size: egui::Vec2,
}

impl<'a> SpriteLabel<'a> {
    /// Create a label
    #[must_use = "You must call .show() to render the label"]
    pub fn new(text: &'a str, font: &'a Handle<BitmapFont>) -> Self {
        Self {
            text,
            font,
            color: egui::Color32::WHITE,
        }
    }

    /// Set the text color
    #[must_use = "You must call .show() to render the label"]
    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = color;

        self
    }

    /// Render the label
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        self.ui(ui)
    }

    /// Calculate the size the text box would take up in the given UI with a specified wrap width.
    ///
    /// Returns `None` if the font data cannot be loaded
    pub fn calculate_layout(
        &self,
        ui: &mut egui::Ui,
        max_width: Option<f32>,
    ) -> Option<SpriteLabelCalculatedLayout> {
        let max_width = max_width.map(|x| x.floor() as usize);

        // Load font data and texture id
        let retro_font_cache_item = {
            let ctx = ui.ctx();
            let Some(val) = ctx.memory_mut(|memory| {
                let retro_font_cache = memory
                    .data
                    .get_temp_mut_or_default::<BitmapFontCache>(egui::Id::NULL)
                    .lock();
                retro_font_cache.get(self.font).cloned()
            }) else {
                return None;
            };
            val
        };
        let font_data = &retro_font_cache_item.font_data;

        // Initialize some helpers
        let font = &font_data.font;
        let default_glyph = font.glyphs.get(&(' ' as u32));

        // Calculate line breaks for the text
        let mut line_breaks = unicode_linebreak::linebreaks(self.text).collect::<Vec<_>>();
        line_breaks.reverse();
        let line_breaks = line_breaks; // Make immutable

        // Create a vector that holds all of the lines of the text and the glyphs in each line
        let mut lines: Vec<Vec<bfn::Glyph>> = Default::default();

        // Start glyph layout
        let mut current_line = Vec::new();
        let mut line_x = 0; // The x position in the line we are currently at
        for (char_i, char) in self.text.char_indices() {
            // Get the glyph for this character
            let glyph = font
                .glyphs
                .get(&(char as u32))
                .or(default_glyph)
                .unwrap_or_else(|| panic!("Font does not contain glyph for character: {:?}", char));

            // Add the next glyph to the current line
            current_line.push(glyph.clone());

            // Wrap the line if necessary
            if let Some(max_width) = max_width {
                // Calculate the new x position of the line after adding this glyph
                line_x += glyph.bounds.width;

                // If this character must break the line
                if line_breaks
                    .iter()
                    .any(|(i, op)| i == &(char_i + 1) && op == &BreakOpportunity::Mandatory)
                    // The last character always breaks, but we want to ignore that one
                    && char_i != self.text.len() - 1
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
                                    .fold(0, |width, g| width + g.bounds.width);
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        // Push the last line
        lines.push(current_line);

        // Determine the size of the label
        let line_height = (font.char_height) as f32;
        let label_height = line_height * lines.len() as f32;
        let label_width = lines.iter().fold(0, |width, line| {
            let line_width = line
                .iter()
                .fold(0, |width, glyph| width + glyph.bounds.width);

            if line_width > width {
                line_width
            } else {
                width
            }
        }) as f32;
        let size = egui::Vec2::new(label_width, label_height);

        Some(SpriteLabelCalculatedLayout {
            lines,
            size,
            font_cache: retro_font_cache_item,
            line_height,
        })
    }

    pub fn paint_at(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        layout: SpriteLabelCalculatedLayout,
    ) {
        // background
        // let mut mesh = Mesh::default();
        // mesh.add_colored_rect(rect, egui::Color32::WHITE);
        // ui.painter().add(mesh);

        let pos = rect.min;

        // Aliase
        let font = &layout.font_cache.font_data.font;

        // Render the meshes for all of the glyphs in our label
        for (line_idx, line) in layout.lines.iter().enumerate() {
            let line_width =
                line.iter()
                    .fold(0, |width, glyph| width + glyph.bounds.width) as f32;
            let mut current_x = 0.0;

            // Calculate horizontal offset to match alignment
            let line_x_offset = match ui.layout().horizontal_align() {
                egui::Align::Min => 0.0,
                egui::Align::Center => (layout.size.x - line_width) / 2.0,
                egui::Align::Max => layout.size.x - line_width,
            };

            for glyph in line {
                let glyph: &bfn::Glyph = glyph;

                // Skip whitespace chars
                if char::from_u32(glyph.code_point).unwrap().is_whitespace() {
                    current_x += font.space_width as f32;
                    continue;
                }

                // Create mesh for glyph
                let mut mesh = egui::Mesh::with_texture(layout.font_cache.texture_id);

                // Calculate glyph position and size
                // let char_y_offset = (glyph.bounds.height as f32) + glyph.bounds.y as f32;
                let glyph_pos = egui::Vec2::new(
                    current_x + line_x_offset,
                    line_idx as f32 * layout.line_height,
                );
                let glyph_size =
                    egui::Vec2::new(glyph.bounds.width as f32, glyph.bounds.height as f32);
                let glyph_rect = egui::Rect::from_min_size(pos + glyph_pos, glyph_size);

                let glyph_uv = egui::Rect::from_min_size(
                    egui::Pos2::new(glyph.bounds.x as f32 / 1856., glyph.bounds.y as f32 / 1856.),
                    egui::Vec2::new(
                        glyph.bounds.width as f32 / 1856.,
                        glyph.bounds.height as f32 / 1856.,
                    ),
                );

                // Add the glyph to the mesh and render it
                mesh.add_rect_with_uv(glyph_rect, glyph_uv, self.color);
                ui.painter().add(mesh);

                // Update the x position
                current_x += glyph.bounds.width as f32;
            }
        }
    }
}

impl<'a> Widget for SpriteLabel<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let empty_response = ui.allocate_response(egui::Vec2::ZERO, egui::Sense::hover());

        // Calculate label layout
        let wrap_width = if ui.wrap_text() {
            Some(ui.available_width())
        } else {
            None
        };
        let layout = if let Some(layout) = self.calculate_layout(ui, wrap_width) {
            layout
        } else {
            return empty_response;
        };

        // Allocate a rect and response for the label
        let (rect, response) = ui.allocate_exact_size(layout.size, egui::Sense::hover());

        // Paint the label
        self.paint_at(ui, rect, layout);

        response
    }
}

/// Extra functions on top of [`egui::Ui`] for retro widgets
pub trait SpriteLabelEguiUiExt {
    fn retro_label(self, text: &str, font: &Handle<BitmapFont>) -> egui::Response;
}

impl SpriteLabelEguiUiExt for &mut egui::Ui {
    fn retro_label(self, text: &str, font: &Handle<BitmapFont>) -> egui::Response {
        SpriteLabel::new(text, font).show(self)
    }
}
