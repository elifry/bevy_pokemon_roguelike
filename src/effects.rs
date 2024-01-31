use bevy::{prelude::*, sprite::Anchor};
use strum::{Display, EnumString};

use crate::{
    graphics::{assets::EffectAssets, get_world_position, PIECE_Z},
    vector2_int::Vector2Int,
    GameState,
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), (spawn_test));
    }
}

#[derive(Debug, Hash, PartialEq, Eq, EnumString, Display, Copy, Clone)]
#[strum()]
pub enum EffectID {
    #[strum(serialize = "0110")]
    _0110,
}

#[derive(Component, Debug)]
pub struct Effect(pub EffectID);

fn spawn_test(mut commands: Commands, assets: Res<EffectAssets>) {
    let texture_atlas = assets
        .0
        .get(&EffectID::_0110)
        .unwrap()
        .textures
        .get("002")
        .unwrap()
        .clone();

    let sprite = TextureAtlasSprite {
        index: 5,
        anchor: Anchor::Center,
        ..default()
    };

    let v = get_world_position(&Vector2Int::new(3, 3), PIECE_Z);

    commands.spawn((
        Name::new("Test"),
        SpriteSheetBundle {
            texture_atlas,
            sprite,
            transform: Transform::from_translation(v),
            ..default()
        },
    ));
}
