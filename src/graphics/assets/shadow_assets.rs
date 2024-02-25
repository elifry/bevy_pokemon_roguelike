use crate::GameState;
use bevy::prelude::*;

use super::AssetsLoading;

const SHADOWS_ASSET: &str = "ui/Shadows.png";

pub struct ShadowAssetsPlugin;

impl Plugin for ShadowAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShadowAssets>()
            .add_systems(OnEnter(GameState::Loading), load_shadow_assets);
    }
}

#[derive(Resource, Debug, Default)]
pub struct ShadowAssets {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
}

fn load_shadow_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut shadow_assets: ResMut<ShadowAssets>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("shadow assets loading...");

    shadow_assets.texture = asset_server.load(SHADOWS_ASSET);
    loading.0.push(shadow_assets.texture.clone().untyped());

    let atlas_layout = TextureAtlasLayout::from_grid(Vec2::new(32., 16.), 3, 14, None, None);
    shadow_assets.atlas_layout = atlases.add(atlas_layout);
    loading.0.push(shadow_assets.atlas_layout.clone().untyped());
}
