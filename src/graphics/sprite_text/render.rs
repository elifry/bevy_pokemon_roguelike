use bevy::{
    prelude::*,
    render::texture::{ImageSampler, ImageType},
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo},
};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

use crate::graphics::assets::font_assets::FontSheet;

use super::SpriteText;

fn extract_sub_image(img: &Image, rect: &Rect) -> Option<RgbaImage> {
    let width: u32 = rect.width() as u32;
    let height: u32 = rect.height() as u32;

    // Validate the rectangle dimensions
    if width == 0
        || height == 0
        || rect.max.x as u32 > img.width()
        || rect.max.y as u32 > img.height()
    {
        return None;
    }

    // Create a new image buffer for the sub-image
    let mut sub_img: RgbaImage = ImageBuffer::new(width, height);

    let atlas_image_width = img.texture_descriptor.size.width;

    info!("atlas width {}", atlas_image_width);

    // Calculate the number of bytes per row (assuming RGBA format, hence * 4)
    let bytes_per_row = atlas_image_width as usize * 4;

    for y in 0..height {
        for x in 0..width {
            let pixel_index = ((rect.min.y + y as f32) as usize * bytes_per_row)
                + ((rect.min.x + x as f32) as usize * 4);

            let red = img.data[pixel_index];
            let green = img.data[pixel_index + 1];
            let blue = img.data[pixel_index + 2];
            let alpha = img.data[pixel_index + 3];

            let rgba: Rgba<u8> = Rgba([red, green, blue, alpha]);

            sub_img.put_pixel(x, y, rgba);
        }
    }

    Some(sub_img)
}

pub(crate) fn render_texture(
    query: Query<(Entity, &SpriteText, &GlobalTransform, &Text2dBounds), Changed<SpriteText>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    font_sheets: Res<Assets<FontSheet>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    for (entity, sprite_text, global_transform, bounds) in query.iter() {
        for section in sprite_text.sections.iter() {
            let font_sheet = font_sheets
                .get(section.font.font_sheet.id())
                .expect("Unable to load the fontsheet for the font");

            let texture_atlas = texture_atlases
                .get(section.font.texture_atlas.id())
                .expect("Unable to load the texture atlas for the font");

            let texture_image = images.get(texture_atlas.texture.id()).unwrap();

            let mut total_width: f32 = 0.;
            let mut max_height: f32 = 0.;

            let glyphs: Vec<_> = section
                .value
                .chars()
                .map(|character| {
                    let glyph_id = character as u32;
                    // TODO: handle glyph not found
                    let glyph = font_sheet.glyphs.get(&glyph_id).unwrap();
                    let glyph_rect = texture_atlas.textures[glyph.index];

                    total_width += glyph_rect.width();
                    max_height = max_height.max(glyph_rect.height());

                    let image = extract_sub_image(texture_image, &glyph_rect)
                        .expect("Failed to extract sub-image");

                    (glyph_id, image, glyph_rect)
                })
                .collect();

            let mut combined = RgbaImage::new(total_width as u32, max_height as u32);

            // Backgrounds
            // let red = Rgba([255, 0, 0, 255]);
            // for pixel in combined.pixels_mut() {
            //     *pixel = red;
            // }

            let mut x_offset: i64 = 0;
            for (_id, image, texture) in glyphs {
                image::imageops::overlay(&mut combined, &image, x_offset, 0);
                x_offset += texture.width() as i64;
            }

            let image = Image::from_dynamic(DynamicImage::ImageRgba8(combined), false);

            let image_handle = images.add(image);

            info!("Renderer texture font");

            commands.entity(entity).insert((
                image_handle,
                Sprite {
                    custom_size: Some(Vec2::new(total_width, max_height)),
                    ..default()
                },
            ));
        }
    }
}
