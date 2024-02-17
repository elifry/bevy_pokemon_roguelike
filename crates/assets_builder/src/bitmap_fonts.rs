use bitmap_font::bfn::{BoundingBox, Font, Glyph};
use image::{DynamicImage, GenericImage, ImageOutputFormat, Rgba, RgbaImage};
use std::{collections::HashMap, fs::File, io::Cursor, path::Path};

use crunch::{Item, PackedItem, Rect, Rotation};

use crate::{atlas::TextureAtlasEntry, utils::list_png_files_in_folder};

pub fn create_bitmap_font(source_directory: &str, output_filename: &str) {
    let font_texture_files = list_png_files_in_folder(source_directory)
        .unwrap_or_else(|_| panic!("Unable to list texture files in {:?}", source_directory));

    let entries: Vec<_> = font_texture_files
        .iter()
        .filter_map(|file| {
            let texture = image::open(file).unwrap().to_rgba8();
            let (w, h) = (texture.width() as usize, texture.height() as usize);

            let path = Path::new(file);
            let file_stem = path.file_stem().and_then(|n| n.to_str())?;
            let id = u16::from_str_radix(file_stem, 16).ok()?;

            Some(Item::new(
                TextureAtlasEntry { texture, id },
                w,
                h,
                Rotation::None,
            ))
        })
        .collect();

    println!("packing {} glyph font images...", entries.len());

    // Try packing all the rectangles
    let font_name = Path::new(output_filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    let dest = Rect::new(0, 0, 64 * 29, 64 * 29);
    match crunch::pack(dest, entries) {
        Ok(all_packed) => {
            // Create a target atlas image to draw the packed images onto
            let mut atlas = RgbaImage::from_pixel(dest.w as u32, dest.h as u32, Rgba([0, 0, 0, 0]));

            let mut glyphs: HashMap<u16, Glyph> = HashMap::with_capacity(all_packed.len());

            // Copy all the packed images onto the target atlas
            for PackedItem { data, rect } in all_packed.iter() {
                atlas
                    .copy_from(&data.texture, rect.x as u32, rect.y as u32)
                    .unwrap();
                // let rect = bevy_math::Rect::new(
                //     rect.x as f32,
                //     rect.y as f32,
                //     (rect.x + rect.w) as f32,
                //     (rect.y + rect.h) as f32,
                // );
                glyphs.insert(
                    data.id,
                    Glyph {
                        code_point: data.id,
                        bounds: BoundingBox {
                            width: rect.w,
                            height: rect.h,
                            x: rect.x,
                            y: rect.y,
                        },
                        colorless: false,
                    },
                );
            }

            println!("exporting `{}`...", output_filename);

            // Export the packed atlas
            atlas.save(format!("{output_filename}-debug.png")).unwrap();

            let mut texture_bytes: Vec<u8> = Vec::new();
            DynamicImage::ImageRgba8(atlas)
                .write_to(&mut Cursor::new(&mut texture_bytes), ImageOutputFormat::Png)
                .expect("Failed to compress atlas image");

            let font_sheet_data = Font {
                size: (dest.w, dest.h),
                name: font_name.to_string(),
                glyph_count: glyphs.len(),
                char_space: 0,
                char_height: 0,
                line_space: 0,
                glyphs,
                texture: texture_bytes,
            };
            let mut font_sheet_file = File::create(output_filename).unwrap();
            let _ = font_sheet_data.save(&mut font_sheet_file);
        }
        Err(_) => {
            panic!("failed to pack images");
        }
    }
}
