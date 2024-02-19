use bitmap_font::bfn::{BoundingBox, Font, Glyph};
use image::{DynamicImage, GenericImage, ImageOutputFormat, Rgba, RgbaImage};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Cursor,
    path::Path,
};

use crunch::{Item, PackedItem, Rect, Rotation};

use crate::{atlas::TextureAtlasEntry, font_data::FontData, utils::list_png_files_in_folder};

pub fn create_bitmap_font(source_directory: &str, output_filename: &str) {
    println!("Start packing font {}", output_filename);
    let font_texture_files = list_png_files_in_folder(source_directory)
        .unwrap_or_else(|_| panic!("Unable to list texture files in {:?}", source_directory));

    let font_data_file = format!("{source_directory}/FontData.xml");
    let font_data_path = Path::new(&font_data_file);
    let font_data_content = fs::read(font_data_path).expect("Failed to read FontData.xml");
    let font_data =
        FontData::parse_from_xml(&font_data_content).expect("Failed to parse FontData.xml");

    let output_path = Path::new(output_filename);
    fs::create_dir_all(output_path.parent().unwrap().to_str().unwrap()).unwrap();

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

            let mut glyphs: HashMap<u32, Glyph> = HashMap::with_capacity(all_packed.len());

            // Copy all the packed images onto the target atlas
            for PackedItem { data, rect } in all_packed.iter() {
                atlas
                    .copy_from(&data.texture, rect.x as u32, rect.y as u32)
                    .unwrap();

                if font_data.colorless.glyphs.contains(&data.id) {
                    println!("{:X} is colorless", data.id);
                }

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
                        colorless: font_data.colorless.glyphs.contains(&data.id),
                    },
                );
            }

            glyphs.insert(
                ' ' as u32,
                Glyph {
                    code_point: ' ' as u32,
                    bounds: BoundingBox {
                        width: 4,
                        height: 12,
                        x: 0,
                        y: 0,
                    },
                    colorless: false,
                },
            );

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
                char_space: font_data.char_space,
                char_height: font_data.char_height,
                space_width: font_data.space_width,
                line_space: font_data.line_space,
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
