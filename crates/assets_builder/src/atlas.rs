use font_atlas::{FontSheetData, GlyphData};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crunch::{Item, PackedItem, Rect, Rotation};
use image::{GenericImage, Rgba, RgbaImage};

use crate::utils::list_png_files_in_folder;

#[derive(Debug, Clone)]
pub struct TextureAtlasEntry<T> {
    pub id: T,
    pub texture: RgbaImage,
}

pub fn create_font_atlas(source_directory: &str, output_filename: &str) {
    let font_texture_files = list_png_files_in_folder(source_directory)
        .unwrap_or_else(|_| panic!("Unable to list texture files in {:?}", source_directory));

    let entries: Vec<_> = font_texture_files
        .iter()
        .filter_map(|file| {
            let texture = image::open(file).unwrap().to_rgba8();
            let (w, h) = (texture.width() as usize, texture.height() as usize);

            let path = Path::new(file);
            let file_stem = path.file_stem().and_then(|n| n.to_str())?;
            let id = u32::from_str_radix(file_stem, 16).ok()?;

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

    let dest = Rect::new(0, 0, 64 * 29, 64 * 29);
    match crunch::pack(dest, entries) {
        Ok(all_packed) => {
            // Create a target atlas image to draw the packed images onto
            let mut atlas = RgbaImage::from_pixel(dest.w as u32, dest.h as u32, Rgba([0, 0, 0, 0]));

            let mut characters: HashMap<u32, GlyphData> = HashMap::with_capacity(all_packed.len());
            // Copy all the packed images onto the target atlas
            for (index, PackedItem { data, rect }) in all_packed.iter().enumerate() {
                atlas
                    .copy_from(&data.texture, rect.x as u32, rect.y as u32)
                    .unwrap();
                let rect = bevy_math::Rect::new(
                    rect.x as f32,
                    rect.y as f32,
                    (rect.x + rect.w) as f32,
                    (rect.y + rect.h) as f32,
                );
                characters.insert(data.id, GlyphData { rect });
            }

            println!("exporting `{}`...", output_filename);

            // Export the packed atlas
            atlas.save(format!("{output_filename}.png")).unwrap();

            let font_sheet_data = FontSheetData {
                width: dest.w,
                height: dest.h,
                characters,
            };
            let mut font_sheet_file =
                File::create(format!("{output_filename}.fontsheet.data")).unwrap();
            let _ = font_sheet_data.save(&mut font_sheet_file);
        }
        Err(_) => {
            panic!("failed to pack images");
        }
    }
}
