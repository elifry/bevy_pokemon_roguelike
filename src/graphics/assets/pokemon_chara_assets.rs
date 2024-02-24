use std::str::FromStr;

use crate::GameState;
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::utils::HashMap;
use char_animation::CharAnimation;
use strum::IntoEnumIterator;

use super::AssetsLoading;

const CHAR_ANIMATION_FOLDER: &str = "chara";

pub struct PokemonCharaAssetsPlugin;

impl Plugin for PokemonCharaAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharaAssetsFolder(default()))
            .init_resource::<PokemonCharaAssets>()
            .add_systems(OnEnter(GameState::Loading), load_assets_folder)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_chara_assets);
    }
}

#[derive(Resource, Debug, Default)]
pub struct PokemonCharaAssets(pub HashMap<String, Handle<CharAnimation>>);

#[derive(Default, Resource)]
struct CharaAssetsFolder(Handle<LoadedFolder>);

fn load_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut chara_assets_folder: ResMut<CharaAssetsFolder>,
) {
    info!("chara assets loading...");
    chara_assets_folder.0 = asset_server.load_folder(CHAR_ANIMATION_FOLDER);
    loading.0.push(chara_assets_folder.0.clone().untyped());
}

fn process_chara_assets(
    chara_assets_folder: ResMut<CharaAssetsFolder>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    mut pokemon_chara_assets: ResMut<PokemonCharaAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let Some::<&LoadedFolder>(chara_folder) = loaded_folder_assets.get(&chara_assets_folder.0)
    else {
        error!("Could'nt load the chara folder");
        return;
    };

    let pokemon_char_animations: HashMap<String, Handle<CharAnimation>> = chara_folder
        .handles
        .iter()
        .map(|handle| {
            let file_stem = handle
                .path()
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            (
                file_stem.to_string(),
                handle.to_owned().typed::<CharAnimation>(),
            )
        })
        .collect::<HashMap<_, _>>();

    pokemon_chara_assets.0 = pokemon_char_animations;

    // Clean up unused resources
    commands.remove_resource::<CharaAssetsFolder>();

    info!("Assets processed");

    next_state.set(GameState::Initializing);
}
