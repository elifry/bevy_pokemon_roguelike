mod layout;
pub mod node;
mod render;
mod section;
pub mod text;
pub mod ui;
mod utils;

use bevy::{prelude::*, transform::TransformSystem, ui::UiSystem};

pub use node::*;
pub use text::*;
pub use ui::*;

use self::render::{new_image_from_default, render_texture, SpriteTextRenderSet};

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
                (new_image_from_default).in_set(SpriteTextRenderSet::Setup),
                (render_texture).in_set(SpriteTextRenderSet::Draw),
            ),
        );
        // .add_systems(PostUpdate, render_ui_texture.after(UiSystem::Layout))
        // .add_systems(
        //     PostUpdate,
        //     measure_sprite_text_system.before(UiSystem::Layout),
        // );
    }
}
