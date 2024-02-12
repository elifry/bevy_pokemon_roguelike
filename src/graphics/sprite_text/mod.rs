mod render;
pub mod text;
mod utils;

use bevy::prelude::*;

pub use text::*;

use self::render::render_texture;

pub struct SpriteTextPlugin;

impl Plugin for SpriteTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_texture);
    }
}
