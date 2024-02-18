mod atlas;
mod bitmap_fonts;
mod utils;

use crate::bitmap_fonts::create_bitmap_font;

const FONT_RAW_FOLDER_PATH: &str = "raw_assets/fonts";

fn main() {
    build_font_atlases();
}

fn build_font_atlases() {
    println!("Building font atlases...");

    // if let Ok(buffer) = fs::read("assets/fonts/text/font.bin") {
    //     let font_sheet = FontSheet::load(&buffer);
    //     println!("{}", font_sheet.width);
    // };

    // fs::create_dir_all("assets/fonts/text").unwrap();
    // create_font_atlas(FONT_RAW_FOLDER_PATH, "assets/fonts/text/font");

    create_bitmap_font(
        &format!("{FONT_RAW_FOLDER_PATH}/text"),
        "assets/fonts/text/text.bfn",
    );
}
