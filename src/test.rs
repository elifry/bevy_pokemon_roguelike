use bevy::prelude::*;

use crate::graphics::assets::font_assets::FontAssets;
use crate::graphics::sprite_text::{SpriteText, Text2DSpriteBundle};
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test);
    }
}

fn spawn_test(font_sheet_assets: Res<FontAssets>, mut commands: Commands) {
    let text_font = font_sheet_assets.0.get("text").unwrap();

    commands
        .spawn(Text2DSpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
            text_anchor: bevy::sprite::Anchor::BottomLeft,
            text: SpriteText::from_section("Hello world!", text_font.clone()),
            ..default()
        })
        .insert(Name::new("TextSprite Test"));
}
