use bevy::{prelude::*, render::render_resource::Extent3d, sprite::Anchor, text::Text2dBounds};
use bitmap_font::{bfn, fonts::BitmapFont};
use image::{ImageBuffer, Rgba, RgbaImage};

use crate::graphics::sprite_text::{glyph_brush::process_glyph_layout, utils::extract_sub_image};

use super::SpriteText;

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
    font_assets: Res<Assets<BitmapFont>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (sprite_text, bounds, image, text_anchor, mut sprite) in query.iter_mut() {
        let sections_data: Vec<_> = sprite_text
            .sections
            .iter()
            .map(|section| {
                let font_asset = font_assets
                    .get(section.style.font.id())
                    .expect("Unable to load the fontsheet for the font");

                let texture_image = images.get(font_asset.data.texture.id()).unwrap();

                (font_asset, texture_image)
            })
            .collect();

        let text_sections = sections_data
            .iter()
            .enumerate()
            .map(|(index, section)| super::glyph_brush::TextSection {
                text: &sprite_text.sections[index].value,
                font: &section.0.data.font,
            })
            .collect::<Vec<_>>();

        let Some(layout) = process_glyph_layout(&text_sections, Some(bounds.size.x as usize))
        else {
            warn!("Failed to calculated glyph layout");
            continue;
        };

        let mut combined = RgbaImage::new(layout.width as u32, layout.height as u32);

        for (line_idx, line) in layout.lines.iter().enumerate() {
            let line_width =
                line.iter()
                    .fold(0, |width, gl| width + gl.glyph.bounds.width) as f32;
            let mut current_x = 0.0;

            // Calculate horizontal offset to match alignment
            let line_x_offset = 0.; // TODO: handle correctly the horizontal alignment

            for glyph_line in line {
                let glyph: &bfn::Glyph = glyph_line.glyph;
                let font: &bfn::Font = glyph_line.font;
                let line_height = font.char_height;
                let texture_image = sections_data[glyph_line.section_index].1;

                // Skip whitespace chars
                if char::from_u32(glyph.code_point).unwrap().is_whitespace() {
                    current_x += font.space_width as f32;
                    continue;
                }

                let glyph_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                    extract_sub_image(texture_image, &glyph.bounds)
                        .expect("Failed to extract sub-image");

                let pos_x: i64 = (current_x + line_x_offset) as i64;
                let pos_y: i64 = (line_idx * line_height as usize) as i64;

                image::imageops::overlay(&mut combined, &glyph_image, pos_x, pos_y);

                // Update the x position
                current_x += glyph.bounds.width as f32;
            }
        }

        // Draw the background
        // TODO draw the background for text section
        // if let Some(background) = sprite_text.background_color {
        //     for pixel in combined.pixels_mut() {
        //         *pixel = Rgba(background.as_rgba_u8());
        //     }
        // }

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
