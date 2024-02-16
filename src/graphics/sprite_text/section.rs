use bevy::asset::AssetId;
use glyph_brush_layout::ab_glyph::{Glyph, PxScale};

use crate::graphics::assets::font_assets::FontSheet;

/// Text to layout together using a font & scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionSpriteText<'a> {
    /// Text to render
    pub text: &'a str,
    /// Position on screen to render text, in pixels from top-left. Defaults to (0, 0).
    pub scale: PxScale,
    /// Font id to use for this section.
    ///
    /// It must be a valid id in the `FontMap` used for layout calls.
    /// The default `FontId(0)` should always be valid.
    pub font_sheet_id: AssetId<FontSheet>,
}

pub trait ToSectionSpriteText {
    fn to_section_text(&self) -> SectionSpriteText<'_>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SectionSpriteGlyph {
    /// The `SectionText` index.
    pub section_index: usize,
    /// The character byte index from the `SectionText` text.
    pub byte_index: usize,
    /// A positioned glyph.
    pub glyph: Glyph,
    /// Font id.
    pub font_sheet_id: AssetId<FontSheet>,
}
