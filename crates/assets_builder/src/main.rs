mod bitmap_fonts;
mod font_data;
mod utils;

use bevy_math::UVec2;

use crate::bitmap_fonts::create_bitmap_font;

const FONT_RAW_FOLDER_PATH: &str = "raw_assets/fonts";

fn main() {
    build_bitmap_fonts();
}

fn build_bitmap_fonts() {
    println!("Building font atlases...");

    // if let Ok(buffer) = fs::read("assets/fonts/text/font.bin") {
    //     let font_sheet = FontSheet::load(&buffer);
    //     println!("{}", font_sheet.width);
    // };

    // fs::create_dir_all("assets/fonts/text").unwrap();
    // create_font_atlas(FONT_RAW_FOLDER_PATH, "assets/fonts/text/font");

    let fonts_to_load = vec![
        ("banner", UVec2::splat(64 * 68)),
        ("blue", UVec2::new(96, 10)),
        ("green", UVec2::new(96, 10)),
        ("text", UVec2::splat(64 * 29)),
        ("yellow", UVec2::new(96, 10)),
    ];
    fonts_to_load.into_iter().for_each(|(font, atlas_size)| {
        let font_dir = format!("{FONT_RAW_FOLDER_PATH}/{font}");
        create_bitmap_font(&font_dir, &format!("assets/fonts/{font}.bfn"), atlas_size);
    });
}
