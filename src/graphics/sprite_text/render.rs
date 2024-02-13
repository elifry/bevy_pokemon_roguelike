use bevy::{prelude::*, render::render_resource::Extent3d, sprite::Anchor, text::Text2dBounds};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

use crate::graphics::{assets::font_assets::FontSheet, sprite_text::utils::extract_sub_image};

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
        for section in sprite_text.sections.iter() {
            let font_sheet = font_sheets
                .get(section.font.font_sheet.id())
                .expect("Unable to load the fontsheet for the font");

            let texture_atlas = texture_atlases
                .get(section.font.texture_atlas.id())
                .expect("Unable to load the texture atlas for the font");

            let texture_image = images.get(texture_atlas.texture.id()).unwrap();

            let (glyphs, max_width, max_height) = calculate_glyph_positions(
                section,
                font_sheet,
                texture_atlas,
                texture_image,
                bounds,
            );

            let mut combined = RgbaImage::new(max_width, max_height);

            // Backgrounds
            let red = Rgba([255, 0, 0, 255]);
            for pixel in combined.pixels_mut() {
                *pixel = red;
            }
            for positioned_glyph in glyphs {
                image::imageops::overlay(
                    &mut combined,
                    &positioned_glyph.image,
                    positioned_glyph.position.x.into(),
                    positioned_glyph.position.y.into(),
                );
            }

            sprite.custom_size = Some(Vec2::new(combined.width() as f32, combined.height() as f32));
            sprite.anchor = *text_anchor;

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
}
