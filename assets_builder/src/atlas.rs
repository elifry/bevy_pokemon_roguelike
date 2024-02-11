use std::fs;

use crunch::{Item, PackedItem, Rect, Rotation};
use image::{GenericImage, Rgba, RgbaImage};

pub fn create_atlas(source_directory: &str, output_filename: &str) {
    let font_texture_files = list_png_files_in_folder(source_directory)
        .unwrap_or_else(|_| panic!("Unable to list texture files in {:?}", source_directory));

    let entries: Vec<_> = font_texture_files
        .iter()
        .map(|file| {
            let texture = image::open(file).unwrap().to_rgba8();
            let (w, h) = (texture.width() as usize, texture.height() as usize);
            // println!("\tloaded: `{}` ({} x {})", file, w, h);
            Item::new(texture, w, h, Rotation::None)
        })
        .collect();

    println!("packing {} images...", entries.len());

    // Try packing all the rectangles
    let dest = Rect::new(0, 0, 64 * 29, 64 * 29);
    match crunch::pack(dest, entries) {
        Ok(all_packed) => {
            // println!("images packed into ({} x {}) rect", w, h);

            // Create a target atlas image to draw the packed images onto
            let mut atlas = RgbaImage::from_pixel(dest.w as u32, dest.h as u32, Rgba([0, 0, 0, 0]));

            // Copy all the packed images onto the target atlas
            for PackedItem { data, rect } in all_packed {
                atlas
                    .copy_from(&data, rect.x as u32, rect.y as u32)
                    .unwrap();
            }

            println!("exporting `{}`...", output_filename);

            // Export the packed atlas
            atlas.save(output_filename).unwrap();
        }
        Err(_) => {
            panic!("failed to pack images");
        }
    }
}

fn list_png_files_in_folder(folder_path: &str) -> std::io::Result<Vec<String>> {
    let mut png_files = Vec::new();

    // Read the directory specified by folder_path
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and its extension is .png
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("png") {
            // Convert the path to a string and add it to the vector
            if let Some(path_str) = path.to_str() {
                png_files.push(path_str.to_string());
            }
        }
    }

    Ok(png_files)
}
