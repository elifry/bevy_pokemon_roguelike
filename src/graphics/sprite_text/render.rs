use bevy::{
    prelude::*,
    render::{render_resource::Extent3d, Extract},
    sprite::Anchor,
    text::Text2dBounds,
    ui::{widget::UiImageSize, ContentSize, ExtractedUiNode, ExtractedUiNodes},
};
use bevy_inspector_egui::egui::ImageSize;
use image::{Rgba, RgbaImage};

use crate::graphics::assets::font_assets::FontSheet;

use super::{glyph_brush::process_glyph, SpriteText};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SpriteTextRenderSet {
    Setup,
    Draw,
}

pub(crate) fn new_image_from_default(
    mut query: Query<&mut Handle<Image>, Added<SpriteText>>,
    mut images: ResMut<Assets<Image>>,
) {
    for mut texture in query.iter_mut() {
        *texture = images.add(Image::default());
    }
}

pub(crate) fn new_ui_image_from_default(
    mut query: Query<&mut UiImage, Added<SpriteText>>,
    mut images: ResMut<Assets<Image>>,
) {
    for mut image in query.iter_mut() {
        image.texture = images.add(Image::default());
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
                    .get(section.style.font.font_sheet.id())
                    .expect("Unable to load the fontsheet for the font");

                let texture_atlas = texture_atlases
                    .get(section.style.font.texture_atlas.id())
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
            let (positioned_glyphs, section_max_width, section_line_height) = process_glyph(
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
        let height = last_glyph_position.y + line_height;

        info!("Creating image of {width}x{height}");
        let mut combined = RgbaImage::new(width, height);

        // Draw the background
        // TODO draw the background for text section
        // if let Some(background) = sprite_text.background_color {
        //     for pixel in combined.pixels_mut() {
        //         *pixel = Rgba(background.as_rgba_u8());
        //     }
        // }

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

#[allow(clippy::type_complexity)]
pub(crate) fn render_ui_texture(
    mut query: Query<
        (&SpriteText, &Node, &mut UiImage),
        Or<(Changed<SpriteText>, Changed<Anchor>, Changed<Text2dBounds>)>,
    >,
    texture_atlases: Res<Assets<TextureAtlas>>,
    font_sheets: Res<Assets<FontSheet>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (sprite_text, node, image) in query.iter_mut() {
        let sections_data: Vec<_> = sprite_text
            .sections
            .iter()
            .map(|section| {
                let font_sheet = font_sheets
                    .get(section.style.font.font_sheet.id())
                    .expect("Unable to load the fontsheet for the font");

                let texture_atlas = texture_atlases
                    .get(section.style.font.texture_atlas.id())
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
            let (positioned_glyphs, section_max_width, section_line_height) = process_glyph(
                &section.value,
                section_data.0,
                section_data.1,
                section_data.2,
                &node.size(),
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
        let height = last_glyph_position.y + line_height;

        info!("Creating ui image of {width}x{height}");

        let mut combined = RgbaImage::new(width, height);

        // Draw the background
        // TODO draw the background for text section
        // if let Some(background) = sprite_text.background_color {
        //     for pixel in combined.pixels_mut() {
        //         *pixel = Rgba(background.as_rgba_u8());
        //     }
        // }

        // Draw the glyphs
        for positioned_glyph in glyphs {
            image::imageops::overlay(
                &mut combined,
                &positioned_glyph.image,
                positioned_glyph.position.x.into(),
                positioned_glyph.position.y.into(),
            );
        }

        // Update the texture
        if let Some(prev_image) = images.get_mut(image.texture.id()) {
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
