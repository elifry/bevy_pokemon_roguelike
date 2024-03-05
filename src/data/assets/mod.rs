use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};
use pokemon_data::PokemonData;

use crate::{loading::AssetsLoading, utils::get_path_from_handle, GameState};

const POKEMON_DATA_PATH: &str = "data/pokemons";

pub struct DataAssetsPlugin;

impl Plugin for DataAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PokemonDataAssetsFolder>()
            .init_resource::<PokemonDataLookup>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(
                OnEnter(GameState::AssetsLoaded),
                process_pokemon_data_assets,
            );
    }
}

#[derive(Default, Resource)]
struct PokemonDataAssetsFolder(Handle<LoadedFolder>);

#[derive(Resource, Debug, Default)]
pub struct PokemonDataLookup(pub HashMap<String, Handle<PokemonData>>);

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut pokemon_data_assets_folder: ResMut<PokemonDataAssetsFolder>,
) {
    info!("pokemon data assets loading...");

    // Visual Effects
    let pokemon_data_folder = asset_server.load_folder(POKEMON_DATA_PATH);
    loading.0.push(pokemon_data_folder.clone().untyped());
    pokemon_data_assets_folder.0 = pokemon_data_folder;
}

fn process_pokemon_data_assets(
    pokemon_data_assets_folder: Res<PokemonDataAssetsFolder>,
    mut pokemon_data_lookup: ResMut<PokemonDataLookup>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut commands: Commands,
) {
    let folder: &LoadedFolder = match loaded_folder_assets.get(&pokemon_data_assets_folder.0) {
        Some(folder) => folder,
        None => {
            error!("Couldn't load the visual effects folder");
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
                return None;
            };

            Some((file_name, data))
        })
        .collect::<HashMap<_, _>>();

    pokemon_data_lookup.0 = pokemon_data;

    commands.remove_resource::<PokemonDataAssetsFolder>();
}
