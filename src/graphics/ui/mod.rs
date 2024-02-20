use bevy::prelude::*;

pub mod bordered_frame;
pub mod sprite_text;

pub use self::bordered_frame::*;
pub use self::sprite_text::*;

use self::sprite_text::SpriteTextPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpriteTextPlugin,));
    }
}
