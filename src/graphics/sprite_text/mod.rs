mod render;
pub mod text;
mod utils;

use bevy::{prelude::*, transform::TransformSystem};

pub use text::*;

use self::render::{new_image_from_default, render_texture, SpriteTextRenderSet};

pub struct SpriteTextPlugin;

impl Plugin for SpriteTextPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (SpriteTextRenderSet::Setup, SpriteTextRenderSet::Draw)
                .chain()
                .after(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            (
                new_image_from_default.in_set(SpriteTextRenderSet::Setup),
                render_texture.in_set(SpriteTextRenderSet::Draw),
            ),
        );
    }
}
