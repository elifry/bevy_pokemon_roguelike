use bevy::asset::LoadState;
use bevy::prelude::*;

use bevy_asset_loader::prelude::*;

use crate::loading::AssetsLoading;
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
        .init_collection::<TileAssets>();
    }
}

// TODO: handle tile map loading in a separated plugin
#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 24,
        tile_size_y = 24,
        columns = 21,
        rows = 24,
        padding_x = 1,
        padding_y = 1,
        offset_x = 1,
        offset_y = 1
    ))]
    pub tile_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "tiles/forest_path_tiles.png")]
    pub forest_path_texture: Handle<Image>,

    #[asset(path = "tiles/amp_plains_tiles.png")]
    pub amp_plains_texture: Handle<Image>,
}
