use bevy::{prelude::*, ui::Node};
use bevy_inspector_egui::egui;

use crate::graphics::ui::sprite_text::text::{SpriteText, SpriteTextTexture};

#[derive(Component, Clone, Debug)]
pub struct SpriteTextEguiImageNode {
    /// The egui texture ID for the image this is mapping to.
    pub egui_texture: egui::TextureId,
    /// The size of the sub-image.
    pub texture_size: egui::Rect,
    /// The size of the entire texture atlas.
    pub atlas_size: egui::Pos2,
    /// The style of the node.
    pub style: Node,
    /// The egui image the should be placed.
    pub egui_image: egui::Image<'static>,
    /// The bevy texture of the image.
    pub texture: SpriteTextTexture,
    /// The image node for bevy UI.
    pub image: ImageNode,
}

impl Default for SpriteTextEguiImageNode {
    fn default() -> Self {
        Self {
            egui_texture: egui::TextureId::default(),
            texture_size: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO),
            atlas_size: egui::Pos2::ZERO,
            style: Node::default(),
            egui_image: egui::Image::from_uri(""),
            texture: SpriteTextTexture::default(),
            image: ImageNode::default(),
        }
    }
}
