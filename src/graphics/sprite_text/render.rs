use bevy::{prelude::*, render::render_resource::Extent3d, sprite::Anchor, text::Text2dBounds};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

use crate::graphics::{assets::font_assets::FontSheet, sprite_text::utils::extract_sub_image};

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

            let mut line_number = 1;
            let mut max_width: u32 = 0;
            let mut max_height: u32 = 0;
            let mut dx: u32 = 0;
            let mut dy: u32 = 0;
            let space_width = 5;
            let mut current_line_width = 0;

            let mut glyphs = Vec::new();
            for character in section.value.chars() {
                if character == '\n' {
                    // Handle explicit line break
                    dx = 0;
                    dy += max_height;
                    max_width = max_width.max(current_line_width);
                    current_line_width = 0;
                    max_height = 0; // Reset max height for the next line
                    line_number += 1;
                    continue;
                }

                let glyph_id = character as u32;
                let (glyph_width, glyph_image) = if character == ' ' {
                    (space_width, ImageBuffer::new(space_width, max_height)) // Use max_height for consistent line height
                } else if let Some(glyph) = font_sheet.glyphs.get(&glyph_id) {
                    let glyph_rect = texture_atlas.textures[glyph.index];
                    let glyph_image = extract_sub_image(texture_image, &glyph_rect)
                        .expect("Failed to extract sub-image");
                    (glyph_image.width(), glyph_image)
                } else {
                    warn!("couldn't find the character '{}'", character);
                    (space_width, ImageBuffer::new(space_width, 0))
                };

                max_height = max_height.max(glyph_image.height());

                // Check if the current glyph exceeds the bounds and a line break is needed
                if bounds.size.x < (dx + glyph_width) as f32 {
                    dx = 0; // Reset dx for the new line
                    dy += max_height; // Move dy to the next line
                    line_number += 1;
                    max_width = max_width.max(current_line_width);
                    current_line_width = 0; // Reset current line width
                    max_height = glyph_image.height(); // Reset max height for the new line
                }

                glyphs.push((glyph_id, glyph_image, dx, dy));

                // Update dx and current line width
                dx += glyph_width;
                current_line_width = current_line_width.max(dx);
            }

            // After iterating through all characters, update max_width for the last line if needed
            max_width = max_width.max(current_line_width);

            let mut combined = RgbaImage::new(max_width, max_height * line_number);
            // Backgrounds
            let red = Rgba([255, 0, 0, 255]);
            for pixel in combined.pixels_mut() {
                *pixel = red;
            }
            for (_id, image, x, y) in glyphs {
                image::imageops::overlay(&mut combined, &image, x.into(), y.into());
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
