//! Bitmap font label widget

use bevy::prelude::Handle;
use bevy_egui::egui::{self, Color32, Widget};
use bitmap_font::{bfn, fonts::BitmapFont, BitmapFontCache, BitmapFontCacheItem};

use super::glyph_brush::{process_glyph_layout, TextSection};

#[derive(Debug, Clone)]
pub struct UISpriteText<'a> {
    pub sections: Vec<UISpriteTextSection<'a>>,
}

impl<'a> UISpriteText<'a> {
    pub fn from_section(value: impl Into<String>, font: &'a Handle<BitmapFont>) -> Self {
        Self {
            sections: vec![UISpriteTextSection::new(value, font)],
        }
    }

    pub fn from_sections(sections: impl IntoIterator<Item = UISpriteTextSection<'a>>) -> Self {
        Self {
            sections: sections.into_iter().collect(),
        }
    }

    /// Returns the total number of chars in all [`SpriteTextSection`]
    pub fn total_chars_count(&self) -> usize {
        self.sections
            .iter()
            .fold(0, |sum, section| sum + section.value.chars().count())
    }
}

#[derive(Debug, Clone)]
pub struct UISpriteTextSection<'a> {
    pub value: String,
    pub font: &'a Handle<BitmapFont>,
    pub color: egui::Color32,
}

impl<'a> UISpriteTextSection<'a> {
    /// Create a new [`SpriteTextSection`].
    pub fn new(value: impl Into<String>, font: &'a Handle<BitmapFont>) -> Self {
        Self {
            value: value.into(),
            font,
            color: Color32::WHITE,
        }
    }
}

pub struct SpriteTextCalculatedLayout {
    pub font_cache_sections: Vec<BitmapFontCacheItem>,
    pub lines: Vec<Vec<LayoutGlyph>>,
    pub size: egui::Vec2,
}

pub struct LayoutGlyph {
    pub glyph: bfn::Glyph,
    pub section_index: usize,
}

impl<'a> UISpriteText<'a> {
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

        let font_cache_sections: Vec<_> = self
            .sections
            .iter()
            .map(|section| {
                let font_cache_item = {
                    let ctx = ui.ctx();
                    let val = ctx
                        .memory_mut(|memory| {
                            let retro_font_cache = memory
                                .data
                                .get_temp_mut_or_default::<BitmapFontCache>(egui::Id::NULL)
                                .lock();
                            retro_font_cache.get(section.font).cloned()
                        })
                        .expect("Failed to load font cache");
                    val
                };
                font_cache_item
            })
            .collect();

        let text_sections = self
            .sections
            .iter()
            .enumerate()
            .map(|(section_index, section)| {
                let text_section = TextSection {
                    text: &section.value,
                    font: &font_cache_sections[section_index].font_data.font,
                };
                text_section
            })
            .collect::<Vec<_>>();

        let Some(mut calculated_layout) = process_glyph_layout(&text_sections, max_width) else {
            return None;
        };

        let size = egui::Vec2::new(calculated_layout.width, calculated_layout.height);
        let lines = calculated_layout
            .lines
            .drain(..)
            .map(|l| {
               l.iter().map(|gl| LayoutGlyph{ glyph: gl.glyph.to_owned(), section_index: gl.section_index }).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Some(SpriteTextCalculatedLayout {
            lines,
            size,
            font_cache_sections,
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

        // Render the meshes for all of the glyphs in our label
        for (line_idx, line) in layout.lines.iter().enumerate() {
            let line_width =
                line.iter()
                    .fold(0, |width, lg| width + lg.glyph.bounds.width) as f32;
            let mut current_x = 0.0;

            // Calculate horizontal offset to match alignment
            let line_x_offset = match ui.layout().horizontal_align() {
                egui::Align::Min => 0.0,
                egui::Align::Center => (layout.size.x - line_width) / 2.0,
                egui::Align::Max => layout.size.x - line_width,
            };

            for layout_glyph in line {
                let font_cache = &layout.font_cache_sections[layout_glyph.section_index];
                let glyph_uvs = &font_cache.font_data.glyph_uvs;
                let font = &font_cache.font_data.font;
                let line_height = font.char_height as f32;
                let glyph: &bfn::Glyph = &layout_glyph.glyph;

                // Skip whitespace chars
                if char::from_u32(glyph.code_point).unwrap().is_whitespace() {
                    current_x += font.space_width as f32;
                    continue;
                }

                // Create mesh for glyph
                let mut mesh = egui::Mesh::with_texture(font_cache.texture_id);

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
                    false => Color32::WHITE, // TODO handle color
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

impl<'a> Widget for UISpriteText<'a> {
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
        UISpriteText::from_section(text, font).show(self)
    }

    fn sprite_text_colored(
        self,
        text: &str,
        color: Color32,
        font: &Handle<BitmapFont>,
    ) -> egui::Response {
        UISpriteText::from_section(text, font).show(self)
    }
}
