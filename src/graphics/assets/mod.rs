use bevy::asset::LoadState;
use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

use crate::GameState;

pub mod binary_data;
pub mod font_assets;
pub mod pokemon_chara_assets;
pub mod shadow_assets;
pub mod ui_assets;
pub mod visual_effect_assets;

use self::binary_data::BinaryDataPlugin;
use self::font_assets::FontAssetsPlugin;
use self::pokemon_chara_assets::PokemonCharaAssetsPlugin;
use self::shadow_assets::ShadowAssetsPlugin;
use self::ui_assets::UIAssetsPlugin;
use self::visual_effect_assets::VisualEffectAssetsPlugin;

pub struct GraphicAssetsPlugin;

impl Plugin for GraphicAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FontAssetsPlugin,
            VisualEffectAssetsPlugin,
            BinaryDataPlugin,
            UIAssetsPlugin,
            PokemonCharaAssetsPlugin,
            ShadowAssetsPlugin,
        ))
        .init_collection::<TileAssets>()
        .init_resource::<AssetsLoading>()
        .add_systems(OnEnter(GameState::Initializing), set_playing)
        .add_systems(
            Update,
            check_assets_loading.run_if(in_state(GameState::Loading)),
        );
    }
}

#[derive(Resource, Default)]
struct AssetsLoading(Vec<UntypedHandle>);

// TODO: handle tile map loading in a separated plugin
#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 24.,
        tile_size_y = 24.,
        columns = 21,
        rows = 24,
        padding_x = 1.,
        padding_y = 1.,
        offset_x = 1.,
        offset_y = 1.
    ))]
    pub tile_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "tiles/forest_path_tiles.png")]
    pub forest_path_texture: Handle<Image>,

    #[asset(path = "tiles/amp_plains_tiles.png")]
    pub amp_plains_texture: Handle<Image>,
}

fn set_playing(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn check_assets_loading(
    mut next_state: ResMut<NextState<GameState>>,
    loading: Res<AssetsLoading>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let mut is_loading: bool = false;

    for asset in loading.0.iter() {
        match asset_server.get_load_state(asset.id()) {
            Some(LoadState::Loading) => {
                is_loading = true;
                break;
            }
            Some(LoadState::Failed) => {
                error!("asset loading error");
            }
            _ => {}
        }
    }

    if is_loading {
        return;
    }

    info!("Assets loaded");
    commands.remove_resource::<AssetsLoading>();
    next_state.set(GameState::AssetsLoaded);
}
