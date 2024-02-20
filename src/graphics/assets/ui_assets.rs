use crate::{graphics::ui::BorderImage, GameState};
use bevy::prelude::*;
use bevy_egui::egui::emath;

pub struct UIAssetsPlugin;

impl Plugin for UIAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_ui_assets);
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UIAssets {
    pub menu: BorderImage,
}

fn load_ui_assets(world: &mut World) {
    info!("ui assets loading...");

    let menu = BorderImage::load_from_world(
        world,
        "ui/MenuBorder.png",
        URect::from_corners(UVec2::new(0, 0), UVec2::new(24, 24)),
        UiRect::axes(Val::Px(8.0), Val::Px(8.0)),
        Some(UVec2::new(120, 72)),
    );

    world.insert_resource(UIAssets { menu })
}
