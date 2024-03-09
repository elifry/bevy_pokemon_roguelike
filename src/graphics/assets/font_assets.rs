use crate::{loading::AssetsLoading, GameState};
use bevy::prelude::*;
use bitmap_font::fonts::BitmapFont;

const FONTS_PATH: &str = "fonts";

pub struct FontAssetsPlugin;

impl Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontAssets>()
            .add_systems(OnEnter(GameState::Loading), load_font_assets);
    }
}

/// Store the font sheet and the texture atlas for a font
#[derive(Resource, Debug, Default, Clone)]
pub struct FontAssets {
    pub text: Handle<BitmapFont>,    // Text
    pub dungeon: Handle<BitmapFont>, // Banner
    pub damage: Handle<BitmapFont>,  // Yellow
    pub heal: Handle<BitmapFont>,    // Green
    pub exp: Handle<BitmapFont>,     // Blue
}

fn load_font_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut font_assets: ResMut<FontAssets>,
) {
    info!("font assets loading...");

    // Fonts
    font_assets.text = asset_server.load(format!("{FONTS_PATH}/text.bfn"));
    loading.0.push(font_assets.text.clone().untyped());

    font_assets.dungeon = asset_server.load(format!("{FONTS_PATH}/banner.bfn"));
    loading.0.push(font_assets.dungeon.clone().untyped());

    font_assets.damage = asset_server.load(format!("{FONTS_PATH}/yellow.bfn"));
    loading.0.push(font_assets.damage.clone().untyped());

    font_assets.heal = asset_server.load(format!("{FONTS_PATH}/green.bfn"));
    loading.0.push(font_assets.heal.clone().untyped());

    font_assets.exp = asset_server.load(format!("{FONTS_PATH}/blue.bfn"));
    loading.0.push(font_assets.exp.clone().untyped());
}
