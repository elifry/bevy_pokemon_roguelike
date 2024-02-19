//! Bitmap font label widget

use bevy::{log::info, prelude::Handle};
use bevy_egui::egui::{self, Color32, Mesh, Widget};
use bitmap_font::{bfn, fonts::BitmapFont, BitmapFontCache, BitmapFontCacheItem};
use unicode_linebreak::BreakOpportunity;

use super::glyph_brush::{process_glyph_layout, TextSection};

pub struct SpriteText<'a> {
    pub text: &'a str,
    pub font: &'a Handle<BitmapFont>,
    pub color: egui::Color32,
}

pub struct SpriteTextCalculatedLayout {
    pub font_cache: BitmapFontCacheItem,
    pub lines: Vec<Vec<bfn::Glyph>>,
    pub size: egui::Vec2,
}

impl<'a> SpriteText<'a> {
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
    ) -> Option<SpriteTextCalculatedLayout> {
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

        let text_section = TextSection {
            text: self.text,
            font: &font_data.font,
        };
        let text_sections = &[text_section];
        let Some(mut calculated_layout) = process_glyph_layout(text_sections, max_width) else {
            return None;
        };

        let size = egui::Vec2::new(calculated_layout.width, calculated_layout.height);
        let lines = calculated_layout
            .lines
            .drain(..)
            .map(|l| l.iter().map(|gl| gl.glyph.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Some(SpriteTextCalculatedLayout {
            lines,
            size,
            font_cache: retro_font_cache_item,
        })
    }

    pub fn paint_at(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        layout: SpriteTextCalculatedLayout,
    ) {
        // background
        // let mut mesh = Mesh::default();
        // mesh.add_colored_rect(rect, egui::Color32::WHITE);
        // ui.painter().add(mesh);

        let pos = rect.min;

        // Aliase
        let font = &layout.font_cache.font_data.font;
        let line_height = font.char_height as f32;
        let glyph_uvs = &layout.font_cache.font_data.glyph_uvs;

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
                let glyph_pos =
                    egui::Vec2::new(current_x + line_x_offset, line_idx as f32 * line_height);
                let glyph_size =
                    egui::Vec2::new(glyph.bounds.width as f32, glyph.bounds.height as f32);
                let glyph_rect = egui::Rect::from_min_size(pos + glyph_pos, glyph_size);

                let glyph_uv = glyph_uvs
                    .get(&glyph.code_point)
                    .unwrap_or(&egui::Rect::NOTHING);

                let color = match glyph.colorless {
                    true => Color32::WHITE,
                    false => self.color,
                };

                // Add the glyph to the mesh and render it
                mesh.add_rect_with_uv(glyph_rect, *glyph_uv, color);
                ui.painter().add(mesh);

                // Update the x position
                current_x += glyph.bounds.width as f32;
            }
        }
    }
}

impl<'a> Widget for SpriteText<'a> {
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
pub trait SpriteTextEguiUiExt {
    fn sprite_text(self, text: &str, font: &Handle<BitmapFont>) -> egui::Response;
    fn sprite_text_colored(
        self,
        text: &str,
        color: Color32,
        font: &Handle<BitmapFont>,
    ) -> egui::Response;
}

impl SpriteTextEguiUiExt for &mut egui::Ui {
    fn sprite_text(self, text: &str, font: &Handle<BitmapFont>) -> egui::Response {
        SpriteText::new(text, font).show(self)
    }

    fn sprite_text_colored(
        self,
        text: &str,
        color: Color32,
        font: &Handle<BitmapFont>,
    ) -> egui::Response {
        SpriteText::new(text, font).color(color).show(self)
    }
}
