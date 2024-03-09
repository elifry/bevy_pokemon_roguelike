use clap::Parser;
use std::path::Path;

use bevy_math::UVec2;

mod bitmap_fonts;
mod char_animations;
mod data;
pub mod utils;

use crate::{
    bitmap_fonts::create_bitmap_font,
    char_animations::create_char_animation,
    data::{create_pokemon_data, spells::create_spell_data},
    utils::list_directories,
};

const FONT_RAW_FOLDER_PATH: &str = "raw_assets/fonts";
const CHAR_ANIMATION_RAW_FOLDER_PATH: &str = "raw_assets/sprites";
const POKEMON_DATA_RAW_FOLDER_PATH: &str = "raw_assets/data/pokemons";
const SPELL_DATA_RAW_FOLDER_PATH: &str = "raw_assets/data/spells";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Build the char animations
    #[arg(short, long, default_value_t = false)]
    char_animation: bool,

    /// Build the bitmap fonts
    #[arg(short, long, default_value_t = false)]
    bitmap_fonts: bool,

    /// Build the pokemon data
    #[arg(short, long, default_value_t = false)]
    pokemon_data: bool,

    /// Build the spell data
    #[arg(short, long, default_value_t = false)]
    spell_data: bool,

    /// Build all the assets
    #[arg(short, long, default_value_t = false)]
    all: bool,
}

fn main() {
    // println!("hello");
    // let char_animation_data = fs::read("assets/chara/0004.chara").unwrap();
    // let char_animation = CharAnimationFile::load(&char_animation_data).unwrap();
    // let anim = char_animation.anim.get(&AnimKey::Idle).unwrap();
    // let texture_buffer = image::load_from_memory(&anim.texture)
    //     .expect("Failed to decompress char animation texture")
    //     .to_rgba8();

    // let _ = texture_buffer.save("test.png");
    let args = Args::parse();

    if args.pokemon_data || args.all {
        build_pokemon_data();
    }
    if args.spell_data || args.all {
        build_spell_data();
    }
    if args.char_animation || args.all {
        build_char_animations();
    }
    if args.bitmap_fonts || args.all {
        build_bitmap_fonts();
    }
}

fn build_pokemon_data() {
    println!("Building pokemon data...");

    let pokemon_raw_data_path = Path::new(POKEMON_DATA_RAW_FOLDER_PATH);
    create_pokemon_data(pokemon_raw_data_path, "assets/data/pokemons");
}

fn build_spell_data() {
    println!("Building spell data...");

    let pokemon_raw_data_path = Path::new(SPELL_DATA_RAW_FOLDER_PATH);
    create_spell_data(pokemon_raw_data_path, "assets/data/spells");
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
