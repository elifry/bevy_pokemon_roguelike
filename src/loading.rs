use crate::{
    graphics::assets::{PokemonAnimationAssets, PokemonAssets, TileAssets},
    GameState,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("tiles.assets.ron")
                .load_collection::<TileAssets>()
                .load_collection::<PokemonAssets>()
                .init_resource::<PokemonAnimationAssets>(),
        );
    }
}
