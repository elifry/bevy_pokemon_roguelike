use bevy::{prelude::*, render::render_resource::Extent3d, sprite::Anchor, text::Text2dBounds};
use image::{Rgba, RgbaImage};

use crate::graphics::assets::font_assets::FontSheet;

use super::{glyph_brush::calculate_glyph_positions, SpriteText};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SpriteTextRenderSet {
    Setup,
    Draw,
}

pub(crate) fn new_image_from_default(
    mut query: Query<&mut Handle<Image>, Added<SpriteText>>,
    mut images: ResMut<Assets<Image>>,
) {
    for mut canvas in query.iter_mut() {
        *canvas = images.add(Image::default());
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn render_texture(
    mut query: Query<
        (
            &SpriteText,
            &Text2dBounds,
            &Handle<Image>,
            &Anchor,
            &mut Sprite,
        ),
        Or<(Changed<SpriteText>, Changed<Anchor>, Changed<Text2dBounds>)>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
    font_sheets: Res<Assets<FontSheet>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (sprite_text, bounds, image, text_anchor, mut sprite) in query.iter_mut() {
        let sections_data: Vec<_> = sprite_text
            .sections
            .iter()
            .map(|section| {
                let font_sheet = font_sheets
                    .get(section.font.font_sheet.id())
                    .expect("Unable to load the fontsheet for the font");

                let texture_atlas = texture_atlases
                    .get(section.font.texture_atlas.id())
                    .expect("Unable to load the texture atlas for the font");

                let texture_image = images.get(texture_atlas.texture.id()).unwrap();

                (font_sheet, texture_atlas, texture_image)
            })
            .collect();

        let mut last_glyph_position = UVec2::ZERO;
        let mut glyphs = Vec::with_capacity(sprite_text.total_chars_count());
        let mut width = 0;
        let mut line_height = 0;

        for (index, section_data) in sections_data.iter().enumerate() {
            let section = &sprite_text.sections[index];
            let (positioned_glyphs, section_max_width, section_line_height) =
                calculate_glyph_positions(
                    &section.value,
                    section_data.0,
                    section_data.1,
                    section_data.2,
                    &bounds.size,
                    Some(last_glyph_position),
                );

            glyphs.extend_from_slice(&positioned_glyphs);

            width = width.max(section_max_width);
            line_height = line_height.max(section_line_height);

            last_glyph_position = glyphs
                .last()
                .map(|glyph| glyph.position + UVec2::new(glyph.image.width(), 0))
                .unwrap_or(UVec2::ZERO);
        }
        info!("Line height : {line_height}");
        let height = last_glyph_position.y + line_height;

        let mut combined = RgbaImage::new(width, height);

        // Draw the background
        if let Some(background) = sprite_text.background {
            for pixel in combined.pixels_mut() {
                *pixel = Rgba(background.as_rgba_u8());
            }
        }

        // Draw the glyphs
        for positioned_glyph in glyphs {
            image::imageops::overlay(
                &mut combined,
                &positioned_glyph.image,
                positioned_glyph.position.x.into(),
                positioned_glyph.position.y.into(),
            );
        }

        // Update the sprite size / anchor
        sprite.custom_size = Some(Vec2::new(combined.width() as f32, combined.height() as f32));
        sprite.anchor = *text_anchor;

        // Update the texture
        if let Some(prev_image) = images.get_mut(image.id()) {
            prev_image.data.clear();
            prev_image.data.extend_from_slice(&combined);
            prev_image.resize(Extent3d {
                width: combined.width(),
                height: combined.height(),
                depth_or_array_layers: 1,
            });
        }
    }
}
