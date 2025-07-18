use bevy::{asset::AssetPath, prelude::*};
use bevy_inspector_egui::bevy_egui::{egui, EguiUserTextures};

pub use ui::*;

pub mod ui;
mod utils;

#[derive(Debug, Clone)]
pub struct BorderImage {
    /// This is the handle to the Bevy image, which keeps the texture from being garbage collected.
    pub handle: Handle<Image>,
    /// This is the egui texture ID for the image.
    pub egui_texture: egui::TextureId,
    /// This is the size of the frame
    pub texture_border_size: UiRect,
    /// This is the size of the texture in pixels
    pub texture_size: URect,
    pub atlas_size: UVec2,
}

impl BorderImage {
    /// Load a border image from the Bevy world
    pub fn load_from_world<'a, P: Into<AssetPath<'a>>>(
        world: &mut World,
        path: P,
        texture_size: URect,
        border_size: UiRect,
        atlas_size: Option<UVec2>,
    ) -> Self {
        // let world = world.cell();
        let asset_server = world.resource::<AssetServer>();
        let handle = asset_server.load(path);

        let mut ctx = world.resource_mut::<EguiUserTextures>();

        let atlas_size = atlas_size.unwrap_or(texture_size.max);

        Self {
            egui_texture: ctx.add_image(handle.clone()),
            handle,
            texture_border_size: border_size,
            texture_size,
            atlas_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BorderImageBackground {
    pub handle: Handle<Image>,
    pub egui_texture: egui::TextureId,
    pub texture_size: URect,
    pub atlas_size: UVec2,
}

impl BorderImageBackground {
    /// Load a border image from the Bevy world
    pub fn load_from_world<'a, P: Into<AssetPath<'a>>>(
        world: &mut World,
        path: P,
        texture_size: URect,
        atlas_size: Option<UVec2>,
    ) -> Self {
        // let world = world.cell();
        let asset_server = world.resource::<AssetServer>();
        let handle = asset_server.load(path);

        let mut ctx = world.resource_mut::<EguiUserTextures>();

        let atlas_size = atlas_size.unwrap_or(texture_size.max);

        Self {
            egui_texture: ctx.add_image(handle.clone()),
            handle,
            texture_size,
            atlas_size,
        }
    }
}
