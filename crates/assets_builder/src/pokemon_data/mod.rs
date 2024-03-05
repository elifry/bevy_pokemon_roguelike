use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use crate::pokemon_data;
use crate::utils::list_files_in_folder;

use self::data::RawPokemonData;

mod data;

pub fn create_pokemon_data(source_directory: &Path, output_directory: &str) {
    let output_directory = Path::new(output_directory);
    fs::create_dir_all(output_directory).unwrap();

    let pokemon_data_files = list_files_in_folder(source_directory, Some("json"))
        .expect("Failed to load pokemon data folder");

    let pokemon_raw_data = pokemon_data_files
        .iter()
        .map(|file| {
            println!("{file}");
            // Read the file to a Vec<u8>
            let mut data = fs::read(file).unwrap();

            // UTF-8 BOM is three bytes: EF BB BF
            if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
                // Remove the first three bytes (the BOM)
                data = data[3..].to_vec();
            }

            data
        })
        .map(|content| RawPokemonData::parse_from_json(&content).unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", pokemon_raw_data);
}
