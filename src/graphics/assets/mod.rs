use bevy::asset::LoadState;
use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

use crate::GameState;

pub mod fonts;
pub mod pokemon_assets;
pub mod visual_effect_assets;

use self::fonts::FontAsset;
use self::fonts::FontAssetsPlugin;
pub use self::pokemon_assets::*;
use self::visual_effect_assets::VisualEffectAssetsFolder;
use self::visual_effect_assets::VisualEffectAssetsPlugin;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PokemonAssetsPlugin,
            FontAssetsPlugin,
            VisualEffectAssetsPlugin,
        ))
        .init_collection::<TileAssets>()
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::Initializing), set_playing)
        .add_systems(
            Update,
            check_assets_loading.run_if(in_state(GameState::Loading)),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
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
    #[asset(path = "tiles/amp_plains_tiles.png")]
    pub amp_plains: Handle<TextureAtlas>,
}

fn set_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut pokemon_assets_folder: ResMut<PokemonAssetsFolder>,
    mut font_asset: ResMut<FontAsset>,
    mut visual_effect_assets_folder: ResMut<VisualEffectAssetsFolder>,
) {
    println!("assets loading...");
    // TODO: Move this part inside plugins

    // Pokemons
    let pokemon_to_load_list = vec!["charmander", "rattata"];
    for pokemon_to_load in pokemon_to_load_list {
        let pokemon_folder = asset_server.load_folder(format!("pokemons/{pokemon_to_load}"));
        pokemon_assets_folder
            .0
            .insert(pokemon_to_load.to_string(), pokemon_folder);
    }

    font_asset.0 = asset_server.load("fonts/Silver.ttf");

    // Effects
    // let effect_to_load_list = vec!["0110"];
    // for effect_to_load in effect_to_load_list {
    //     let effect_folder = asset_server.load_folder(format!("effects/{effect_to_load}"));
    //     effect_assets_folder
    //         .0
    //         .insert(effect_to_load.to_string(), effect_folder);
    // }

    // Visual Effects
    let visual_effect_folder = asset_server.load_folder("visual_effects");
    visual_effect_assets_folder.0 = visual_effect_folder;
}

// TODO: refacto the assets loading part in each individual plugins using event emitter
fn check_assets_loading(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    tile_assets: Res<TileAssets>,
    pokemon_assets: Res<PokemonAssetsFolder>,
    visual_effects_assets: Res<VisualEffectAssetsFolder>,
    font_asset: Res<FontAsset>,
) {
    let mut is_loading: bool = false;

    // Pokemon assets
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

    // Fonts
    match asset_server.get_load_state(font_asset.0.id()) {
        Some(LoadState::Loading) => {
            is_loading = true;
        }
        Some(LoadState::Failed) => {
            error!("couldn't load visual effect")
        }
        _ => {}
    }

    // Visual effects
    match asset_server.get_load_state(visual_effects_assets.0.id()) {
        Some(LoadState::Loading) => {
            is_loading = true;
        }
        Some(LoadState::Failed) => {
            error!("couldn't load visual effect")
        }
        _ => {}
    }

    // Map tiles
    match asset_server.get_load_state(tile_assets.forest_path.id()) {
        Some(LoadState::Loading) => {
            is_loading = true;
        }
        Some(LoadState::Failed) => {}
        _ => {}
    }

    if is_loading {
        return;
    }
    info!("Assets loaded");
    next_state.set(GameState::AssetsLoaded);
}
