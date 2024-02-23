mod bitmap_fonts;
mod char_animations;
pub mod utils;

use std::{fs, path::Path};

use bevy_math::UVec2;

use crate::{
    bitmap_fonts::create_bitmap_font, char_animations::create_char_animation,
    utils::list_directories,
};

const FONT_RAW_FOLDER_PATH: &str = "raw_assets/fonts";
const CHAR_ANIMATION_RAW_FOLDER_PATH: &str = "raw_assets/sprites";

fn main() {
    // println!("hello");
    // let char_animation_data = fs::read("assets/chara/0001.chara").unwrap();
    // let char_animation = CharAnimation::load(&char_animation_data);
    build_char_animations();
    build_bitmap_fonts();
}

fn build_char_animations() {
    println!("Building char animations...");

    let char_animations_path = Path::new(CHAR_ANIMATION_RAW_FOLDER_PATH);
    for char_directory in list_directories(char_animations_path).unwrap() {
        let file_name = char_directory.file_name().unwrap().to_str().unwrap();
        create_char_animation(&char_directory, &format!("assets/chara/{file_name}.chara"))
    }
}

fn build_bitmap_fonts() {
    println!("Building bitmap fonts...");

    let fonts_to_load = vec![
        ("banner", UVec2::splat(64 * 68)),
        ("blue", UVec2::new(96, 10)),
        ("green", UVec2::new(96, 10)),
        ("text", UVec2::splat(64 * 29)),
        ("yellow", UVec2::new(96, 10)),
    ];
    fonts_to_load.into_iter().for_each(|(font, atlas_size)| {
        let font_dir = format!("{FONT_RAW_FOLDER_PATH}/{font}");
        let font_dir = Path::new(&font_dir);
        create_bitmap_font(font_dir, &format!("assets/fonts/{font}.bfn"), atlas_size);
    });
}
