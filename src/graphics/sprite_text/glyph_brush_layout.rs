use glyph_brush_layout::{Layout, LineBreaker, SectionGeometry, SectionGlyph};

use crate::graphics::assets::font_assets::FontSheet;

use super::section::ToSectionSpriteText;

// pub fn calculate_glyphs<L, S>(
//     font_sheets: &[FontSheet],
//     geometry: &SectionGeometry,
//     sections: &[S],
//     layout: Layout<L>,
// ) -> Vec<SectionGlyph>
// where
//     S: ToSectionSpriteText,
//     L: LineBreaker,
// {
//     let SectionGeometry {
//         screen_position,
//         bounds: (bound_w, bound_h),
//         ..
//     } = *geometry;
// }
