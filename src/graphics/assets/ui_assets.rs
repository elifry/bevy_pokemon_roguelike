use crate::{
    graphics::ui::{BorderImage, BorderImageBackground},
    GameState,
};
use bevy::prelude::*;

pub struct UIAssetsPlugin;

impl Plugin for UIAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_ui_assets);
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UIAssets {
    pub panel_blue: BorderImage,
    pub panel_green: BorderImage,
    pub panel_pink: BorderImage,
    pub transparent_panel_bg: BorderImageBackground,
    pub dark_panel_bg: BorderImageBackground,
}

fn load_ui_assets(world: &mut World) {
    info!("ui assets loading...");

    let background_atlas_size = UVec2::new(24, 48);
    let dark_panel_bg = BorderImageBackground::load_from_world(
        world,
        "ui/MenuBG.png",
        URect::from_corners(UVec2::ZERO, UVec2::new(24, 24)),
        Some(background_atlas_size),
    );

    let transparent_panel_bg = BorderImageBackground::load_from_world(
        world,
        "ui/MenuBG.png",
        URect::from_corners(UVec2::new(0, 24), UVec2::new(24, 48)),
        Some(background_atlas_size),
    );

    let panel_atlas_size = UVec2::new(120, 72);
    let panel_green = BorderImage::load_from_world(
        world,
        "ui/MenuBorder.png",
        URect::from_corners(UVec2::new(0, 0), UVec2::new(24, 24)),
        UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
        Some(panel_atlas_size),
    );

    let panel_blue_pos = UVec2::new(0, 24);
    let panel_blue = BorderImage::load_from_world(
        world,
        "ui/MenuBorder.png",
        URect::from_corners(panel_blue_pos, UVec2::new(24, 24) + panel_blue_pos),
        UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
        Some(panel_atlas_size),
    );

    let panel_pink_pos = UVec2::new(0, 48);
    let panel_pink = BorderImage::load_from_world(
        world,
        "ui/MenuBorder.png",
        URect::from_corners(panel_pink_pos, UVec2::new(24, 24) + panel_pink_pos),
        UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
        Some(panel_atlas_size),
    );

    world.insert_resource(UIAssets {
        panel_green,
        panel_blue,
        panel_pink,
        transparent_panel_bg,
        dark_panel_bg,
    })
}
