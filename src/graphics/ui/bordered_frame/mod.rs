use bevy::{asset::AssetPath, prelude::*};
use bevy_egui::egui;

pub mod ui;

#[derive(Debug, Clone)]
pub struct BorderImage {
    /// This is the handle to the Bevy image, which keeps the texture from being garbage collected.
    pub handle: Handle<Image>,
    /// This is the egui texture ID for the image.
    pub egui_texture: egui::TextureId,
    /// This is the size of the frame
    pub texture_border_size: UiRect,
    /// This is the size of the texture in pixels
    pub texture_size: UVec2,
}

impl BorderImage {
    /// Load a border image from the Bevy world
    pub fn load_from_world<'a, P: Into<AssetPath<'a>>>(
        world: &mut World,
        path: P,
        image_size: UVec2,
        border_size: UiRect,
    ) -> Self {
        let world = world.cell();
        let asset_server = world.resource::<AssetServer>();
        let handle = asset_server.load(path);

        let mut ctx = world.resource_mut::<bevy_egui::EguiUserTextures>();

        Self {
            egui_texture: ctx.add_image(handle.clone()),
            handle,
            texture_border_size: border_size,
            texture_size: image_size,
        }
    }
}
