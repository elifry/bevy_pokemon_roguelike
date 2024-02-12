use bevy::prelude::*;
use bevy::sprite::SpriteSheetBundle;

use crate::graphics::assets::font_assets::{FontAssets, FontSheet};
use crate::graphics::sprite_text::{SpriteText, Text2DSpriteBundle};
use crate::GameState;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test);
    }
}

fn spawn_test(
    font_sheet_assets: Res<FontAssets>,
    font_sheets: Res<Assets<FontSheet>>,
    mut commands: Commands,
) {
    let font_asset = font_sheet_assets.0.get("text").unwrap();
    let font_sheet = font_sheets.get(font_asset.font_sheet.id()).unwrap();
    let character: u32 = 'A' as u32;
    let glyph = font_sheet.glyphs.get(&character).unwrap();

    // commands.spawn(SpriteSheetBundle {
    //     texture_atlas: font_asset.texture_atlas.clone(),
    //     transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
    //     sprite: TextureAtlasSprite {
    //         index: glyph.index,
    //         ..default()
    //     },
    //     ..default()
    // });
    //let a = SpriteBundle;

    commands
        .spawn(Text2DSpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 20.)),
            text: SpriteText::from_section("Test", font_asset.clone()),
            ..default()
        })
        .insert(Name::new("TextSprite Test"));
}
