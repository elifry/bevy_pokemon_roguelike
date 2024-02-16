use bevy::{
    asset::{AssetId, Assets},
    math::Vec2,
    text::{scale_value, Text, TextAlignment, TextError},
};
use glyph_brush_layout::{ab_glyph::PxScale, SectionGeometry, SectionText, ToSectionText};

use crate::graphics::assets::font_assets::{FontAssets, FontSheet};

use super::{
    section::{SectionSpriteText, ToSectionSpriteText},
    SpriteText,
};

#[derive(Debug, Clone)]
pub struct SpriteTextMeasureSection {
    pub text: Box<str>,
    pub scale: f32,
    pub font_sheet_id: AssetId<FontSheet>,
}

#[derive(Debug, Clone, Default)]
pub struct SpriteTextMeasureInfo {
    pub font_sheets: Box<[FontSheet]>,
    pub sections: Box<[SpriteTextMeasureSection]>,
    pub justification: TextAlignment,
    pub linebreak_behavior: glyph_brush_layout::BuiltInLineBreaker,
    pub min: Vec2,
    pub max: Vec2,
}

impl SpriteTextMeasureInfo {
    pub fn from_text(
        text: &SpriteText,
        font_sheets: &Assets<FontSheet>,
        scale_factor: f32,
    ) -> Result<SpriteTextMeasureInfo, TextError> {
        let sections = &text.sections;
        for section in sections {
            if !font_sheets.contains(&section.style.font.font_sheet) {
                return Err(TextError::NoSuchFont);
            }
        }

        let (auto_font_sheets, sections) = sections
            .iter()
            .enumerate()
            .map(|(i, section)| {
                // SAFETY: we exited early earlier in this function if
                // one of the fonts was missing.
                let font_sheet_id = section.style.font.font_sheet.id();
                let font_sheet = unsafe { font_sheets.get(font_sheet_id).unwrap_unchecked() };
                (
                    font_sheet.clone(),
                    SpriteTextMeasureSection {
                        font_sheet_id,
                        scale: scale_value(section.style.font_size, scale_factor.into()),
                        text: section.value.clone().into_boxed_str(),
                    },
                )
            })
            .unzip();

        Ok(Self::new(
            auto_font_sheets,
            sections,
            text.alignment,
            text.linebreak_behavior.into(),
        ))
    }
    fn new(
        font_sheets: Vec<FontSheet>,
        sections: Vec<SpriteTextMeasureSection>,
        justification: TextAlignment,
        linebreak_behavior: glyph_brush_layout::BuiltInLineBreaker,
    ) -> Self {
        let mut info = Self {
            font_sheets: font_sheets.into_boxed_slice(),
            sections: sections.into_boxed_slice(),
            justification,
            linebreak_behavior,
            min: Vec2::ZERO,
            max: Vec2::ZERO,
        };

        let min = info.compute_size(Vec2::new(0.0, f32::INFINITY));
        let max = info.compute_size(Vec2::INFINITY);
        info.min = min;
        info.max = max;
        info
    }

    pub fn compute_size(&self, bounds: Vec2) -> Vec2 {
        let sections = &self.sections;
        let geom = SectionGeometry {
            bounds: (bounds.x, bounds.y),
            ..Default::default()
        };
        let layout = glyph_brush_layout::Layout::default()
            .h_align(self.justification.into())
            .line_breaker(self.linebreak_behavior);

        // let section_glyphs = calculate_glyphs(&self.font_sheets, &geom, sections, layout);

        // TODO compute the size of the font
        // compute_text_bounds(self.sections)
        todo!()
    }
}

impl ToSectionSpriteText for SpriteTextMeasureSection {
    #[inline(always)]
    fn to_section_text(&self) -> SectionSpriteText<'_> {
        SectionSpriteText {
            text: &self.text,
            scale: PxScale::from(self.scale),
            font_sheet_id: self.font_sheet_id,
        }
    }
}
