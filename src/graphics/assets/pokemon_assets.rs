use std::str::FromStr;

use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

use crate::graphics::anim_data::{AnimData, AnimKey};
use crate::pokemons::Pokemons;
use crate::GameState;

pub struct PokemonAssetsPlugin;

impl Plugin for PokemonAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PokemonAssetsFolder(default()))
            .init_resource::<PokemonAnimationAssets>()
            .add_systems(OnEnter(GameState::AssetsLoaded), process_pokemon_assets);
    }
}

#[derive(Resource, Debug, Default)]
pub struct PokemonAnimationAssets(pub HashMap<Pokemons, PokemonAnimation>);

#[derive(Default, Resource)]
pub struct PokemonAssetsFolder(pub HashMap<String, Handle<LoadedFolder>>);

#[derive(Debug, Clone)]
pub struct PokemonAnimation {
    pub textures: HashMap<AnimKey, Handle<TextureAtlas>>,
    pub anim_data: Handle<AnimData>,
}

fn process_pokemon_assets(
    pokemon_assets: ResMut<PokemonAssetsFolder>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    anim_data_assets: Res<Assets<AnimData>>,
    mut pokemon_animation_assets: ResMut<PokemonAnimationAssets>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
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

        let anim_to_load = vec![
            AnimKey::Idle,
            AnimKey::Walk,
            AnimKey::Attack,
            AnimKey::Hurt,
            AnimKey::Swing,
        ];

        let mut anim_textures: HashMap<AnimKey, Handle<TextureAtlas>> = HashMap::new();

        for anim_key in anim_to_load {
            let texture = get_texture_atlas_by_anim_key(
                anim_key,
                anim_data,
                &mut hashmap_files,
                &mut texture_atlasses,
            );
            anim_textures.insert(anim_key, texture);
        }

        let pokemon_animation = PokemonAnimation {
            textures: anim_textures,
            anim_data: anim_data_handle,
        };

        pokemon_animation_assets
            .0
            .insert(pokemon, pokemon_animation);
    }

    // Clean up unused resources
    commands.remove_resource::<PokemonAssetsFolder>();

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
