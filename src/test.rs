use bevy::prelude::*;
use bevy::sprite::{SpriteBundle, SpriteSheetBundle};

use crate::graphics::assets::font_assets::FontSheetAsset;
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test);
    }
}

fn spawn_test(font_sheets: Res<FontSheetAsset>, mut commands: Commands) {
    let font_sheet = font_sheets.0.get("text").unwrap();
    let character: u32 = 'A' as u32;
    let glyph = font_sheet.glyphs.get(&character).unwrap();

    commands.spawn(SpriteSheetBundle {
        texture_atlas: font_sheet.texture_atlas.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
        sprite: TextureAtlasSprite {
            index: glyph.index,
            ..default()
        },
        ..default()
    });
}
