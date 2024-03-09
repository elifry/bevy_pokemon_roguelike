use crate::utils::list_files_in_folder;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

use self::data::RawPokemonData;

mod data;

pub fn create_pokemon_data(source_directory: &Path, output_directory: &str) {
    let output_directory = Path::new(output_directory);
    fs::create_dir_all(output_directory).unwrap();

    let pokemon_data_files = list_files_in_folder(source_directory, Some("json"))
        .expect("Failed to load pokemon data folder");

    let pokemon_data = pokemon_data_files
        .iter()
        .map(|file| {
            let path = Path::new(file);

            // Read the file to a Vec<u8>
            let mut data = fs::read(path).unwrap();

            // UTF-8 BOM is three bytes: EF BB BF
            if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
                // Remove the first three bytes (the BOM)
                data = data[3..].to_vec();
            }

            (path, data)
        })
        .map(|(path, content)| (path, RawPokemonData::parse_from_json(&content).unwrap()))
        .map(|(path, data)| (path, data.to_data()))
        .collect::<HashMap<_, _>>();

    for (path, data) in pokemon_data {
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let file_name = format!("{file_stem}.pd.ron");

        let output_file = output_directory.join(file_name);
        let mut output_file = File::create(output_file).unwrap();
        let _ = data.save(&mut output_file);
    }
}
