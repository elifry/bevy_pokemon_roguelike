mod glyph_brush;
pub mod node;
mod render;
pub mod text;
mod utils;

use bevy::{prelude::*, transform::TransformSystem, ui::UiSystem};

pub use node::*;
pub use text::*;

use self::render::{
    measure_sprite_text_system, new_image_from_default, new_ui_image_from_default, render_texture,
    render_ui_texture, SpriteTextRenderSet,
};

pub struct SpriteTextPlugin;

impl Plugin for SpriteTextPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                SpriteTextRenderSet::Setup
                    .before(UiSystem::Layout)
                    .before(TransformSystem::TransformPropagate),
                (SpriteTextRenderSet::Draw)
                    .after(SpriteTextRenderSet::Setup)
                    .after(TransformSystem::TransformPropagate),
            ),
        )
        .add_systems(
            PostUpdate,
            (
                (new_image_from_default, new_ui_image_from_default)
                    .in_set(SpriteTextRenderSet::Setup),
                (render_texture).in_set(SpriteTextRenderSet::Draw),
            ),
        )
        .add_systems(PostUpdate, render_ui_texture.after(UiSystem::Layout))
        .add_systems(
            PostUpdate,
            measure_sprite_text_system.before(UiSystem::Layout),
        );
    }
}
