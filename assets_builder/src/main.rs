mod atlas;

use std::fs;

use crate::atlas::create_atlas;

const FONT_RAW_FOLDER_PATH: &str = "assets/fonts/text";

fn main() {
    build_font_atlases();
}

fn build_font_atlases() {
    println!("Building font atlases...");

    fs::create_dir_all("target/output").unwrap();

    create_atlas(FONT_RAW_FOLDER_PATH, "target/output/atlas.png");
}
