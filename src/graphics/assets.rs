use std::str::FromStr;

use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

use crate::graphics::anim_data::AnimData;
use crate::pokemons::Pokemons;
use crate::GameState;

use super::anim_data::AnimKey;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<TileAssets>()
            .insert_resource(PokemonAssetsFolder(default()))
            .init_resource::<PokemonAnimationAssets>()
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_assets)
            .add_systems(OnEnter(GameState::Initializing), set_playing)
            .add_systems(
                Update,
                check_assets_loading.run_if(in_state(GameState::Loading)),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    // #[asset(key = "tiles.forest_path")]
    #[asset(texture_atlas(
        tile_size_x = 24.,
        tile_size_y = 24.,
        columns = 21,
        rows = 24,
        padding_x = 1.,
        padding_y = 1.,
        offset_x = 1.,
        offset_y = 1.
    ))]
    #[asset(path = "tiles/forest_path_tiles.png")]
    pub forest_path: Handle<TextureAtlas>,
}

#[derive(Resource, Debug, Default)]
pub struct PokemonAnimationAssets(pub HashMap<Pokemons, PokemonAnimation>);

#[derive(Default, Resource)]
pub struct PokemonAssetsFolder(pub HashMap<String, Handle<LoadedFolder>>);

#[derive(Debug, Clone)]
pub struct PokemonAnimation {
    pub idle: Handle<TextureAtlas>,
    pub walk: Handle<TextureAtlas>,
    pub attack: Handle<TextureAtlas>,
    pub anim_data: Handle<AnimData>,
}

fn set_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut pokemon_assets_folder: ResMut<PokemonAssetsFolder>,
) {
    println!("asset loading...");
    let pokemon_to_load_list = vec!["charmander", "rattata"];

    for pokemon_to_load in pokemon_to_load_list {
        let pokemon_folder = asset_server.load_folder(format!("pokemons/{pokemon_to_load}"));
        pokemon_assets_folder
            .0
            .insert(pokemon_to_load.to_string(), pokemon_folder);
    }
}

fn check_assets_loading(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    tile_assets: Res<TileAssets>,
    pokemon_assets: Res<PokemonAssetsFolder>,
) {
    let mut is_loading: bool = false;
    for (_pokemon, asset) in pokemon_assets.0.iter() {
        match asset_server.get_load_state(asset.id()) {
            Some(LoadState::Loading) => {
                is_loading = true;
                break;
            }
            Some(LoadState::Failed) => {
                // error!("asset loading error");
            }
            _ => {}
        }
    }

    match asset_server.get_load_state(tile_assets.forest_path.id()) {
        Some(LoadState::Loading) => {
            is_loading = true;
        }
        Some(LoadState::Failed) => {
            //error!("asset loading error");
        }
        _ => {}
    }

    if is_loading {
        return;
    }
    info!("Assets loaded");
    next_state.set(GameState::AssetsLoaded);
}

fn process_assets(
    pokemon_assets: Res<PokemonAssetsFolder>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    anim_data_assets: Res<Assets<AnimData>>,
    mut pokemon_animation_assets: ResMut<PokemonAnimationAssets>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (pokemon, handle_folder) in pokemon_assets.0.iter() {
        let Some::<&LoadedFolder>(folder) = loaded_folder_assets.get(handle_folder) else {
            error!("Could'nt load the folder for {}", pokemon);
            continue;
        };

        let pokemon = Pokemons::from_str(pokemon).unwrap();

        let mut hashmap_files: HashMap<&str, &UntypedHandle> = folder
            .handles
            .iter()
            .map(|handle| {
                let file_name = handle
                    .path()
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                (file_name, handle)
            })
            .collect::<HashMap<_, _>>();

        let Some(anim_data_handle) = hashmap_files
            .get_mut("AnimData.anim.xml")
            .map(|handle| handle.to_owned().typed::<AnimData>())
        else {
            panic!("Couldn't load the anim data asset for {pokemon}")
        };

        let anim_data = anim_data_assets.get(&anim_data_handle).unwrap();

        let idle_texture_handle = get_texture_atlas_by_anim_key(
            AnimKey::Idle,
            anim_data,
            &mut hashmap_files,
            &mut texture_atlasses,
        );

        let walk_texture_handle = get_texture_atlas_by_anim_key(
            AnimKey::Walk,
            anim_data,
            &mut hashmap_files,
            &mut texture_atlasses,
        );

        let attack_texture_handle = get_texture_atlas_by_anim_key(
            AnimKey::Attack,
            anim_data,
            &mut hashmap_files,
            &mut texture_atlasses,
        );

        let pokemon_animation = PokemonAnimation {
            idle: idle_texture_handle,
            walk: walk_texture_handle,
            attack: attack_texture_handle,
            anim_data: anim_data_handle,
        };

        pokemon_animation_assets
            .0
            .insert(pokemon, pokemon_animation);
    }

    info!("Assets processed");
    next_state.set(GameState::Initializing);
}

fn get_texture_atlas_by_anim_key(
    anim_key: AnimKey,
    anim_data: &AnimData,
    hashmap_files: &mut HashMap<&str, &UntypedHandle>,
    texture_atlasses: &mut ResMut<'_, Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let anim_key_str: &'static str = anim_key.into();
    let mut anim_file = anim_key_str.to_owned();
    anim_file.push_str("-Anim.png");

    let anim_file = anim_file.as_str();

    let Some(idle_anim_handle) = hashmap_files
        .get_mut(anim_file)
        .map(|handle| handle.to_owned().typed::<Image>())
    else {
        panic!("Couldn't load the {anim_key} animation asset")
    };

    let idle_anim_info = anim_data.get(anim_key);

    let idle_texture_atlas = TextureAtlas::from_grid(
        idle_anim_handle,
        idle_anim_info.tile_size(),
        idle_anim_info.columns(),
        idle_anim_info.rows(),
        None,
        None,
    );

    texture_atlasses.add(idle_texture_atlas)
}
