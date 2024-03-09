use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};
use pokemon_data::PokemonData;

use crate::{
    data::assets::text_data::TextAsset, loading::AssetsLoading, utils::get_path_from_handle,
    GameState,
};

const POKEMON_DATA_PATH: &str = "data/pokemons";
const POKEMON_CONVERSION_PATH: &str = "conversions/pokemon.txt";

pub struct PokemonDataPlugin;

impl Plugin for PokemonDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PokemonDataAssetsFolder>()
            .init_resource::<PokemonDataLookup>()
            .init_resource::<PokemonConversion>()
            .init_resource::<PokemonConversionText>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(
                OnEnter(GameState::AssetsLoaded),
                process_pokemon_data_assets,
            );
    }
}

#[derive(Default, Resource)]
struct PokemonDataAssetsFolder(Handle<LoadedFolder>);

#[derive(Default, Resource)]
struct PokemonConversionText(Handle<TextAsset>);

#[derive(Resource, Debug, Default)]
pub struct PokemonDataLookup(pub HashMap<u32, Handle<PokemonData>>);

/// A bidirectional map for ID / pokemon name conversion
#[derive(Resource, Debug, Default)]
pub struct PokemonConversion(pub bimap::BiMap<u32, String>);

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut pokemon_data_assets_folder: ResMut<PokemonDataAssetsFolder>,
    mut pokemon_conversion_text: ResMut<PokemonConversionText>,
) {
    info!("pokemon data assets loading...");

    // Conversion
    // TODO: Create a specific type for the conversion
    pokemon_conversion_text.0 = asset_server.load(POKEMON_CONVERSION_PATH);
    loading.0.push(pokemon_conversion_text.0.clone().untyped());

    // Pokemon Data
    let pokemon_data_folder = asset_server.load_folder(POKEMON_DATA_PATH);
    loading.0.push(pokemon_data_folder.clone().untyped());
    pokemon_data_assets_folder.0 = pokemon_data_folder;
}

fn process_pokemon_data_assets(
    pokemon_data_assets_folder: Res<PokemonDataAssetsFolder>,
    mut pokemon_data_lookup: ResMut<PokemonDataLookup>,
    pokemon_conversion_text: Res<PokemonConversionText>,
    mut pokemon_conversion: ResMut<PokemonConversion>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    text_assets: Res<Assets<TextAsset>>,
    mut commands: Commands,
) {
    let Some(pokemon_conversion_text) = text_assets.get(&pokemon_conversion_text.0) else {
        error!("Failed to load pokemon.txt conversion");
        return;
    };

    // Split the string into lines and iterate over each line
    pokemon_conversion_text.0.lines().for_each(|line| {
        // Split each line by tab and collect into a Vec
        let parts: Vec<&str> = line.split('\t').collect();

        // Parse the first part as u32 for ID and the second part as String for name
        pokemon_conversion.0.insert(
            parts[0].parse::<u32>().expect("Failed to parse ID"),
            parts[1].to_string(),
        );
    });

    let folder: &LoadedFolder = match loaded_folder_assets.get(&pokemon_data_assets_folder.0) {
        Some(folder) => folder,
        None => {
            error!("Couldn't load the pokemon data folder");
            return;
        }
    };

    let pokemon_data = folder
        .handles
        .iter()
        .filter_map(|handle| {
            let Some(path) = get_path_from_handle(handle) else {
                return None;
            };

            let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
                return None;
            };

            let Some(index) = file_name.find('.') else {
                return None;
            };
            // Slice the string from the start up to the position of the first period
            let file_name = file_name[..index].to_string();

            let Ok(data) = handle.clone().try_typed::<PokemonData>() else {
                warn!("Failed to get pokemon data for {file_name}");
                return None;
            };

            let Some(id) = pokemon_conversion.0.get_by_right(&file_name) else {
                warn!("Failed to find pokemon ID for {file_name}");
                return None;
            };

            Some((*id, data))
        })
        .collect::<HashMap<_, _>>();

    pokemon_data_lookup.0 = pokemon_data;

    commands.remove_resource::<PokemonDataAssetsFolder>();
}
