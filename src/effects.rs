use bevy::prelude::*;
use strum::{Display, EnumString};

use crate::{map::Position, vector2_int::Vector2Int, GameState};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test_effect);
    }
}

#[derive(Component, Debug, Hash, PartialEq, Eq, EnumString, Display, Copy, Clone)]
pub enum Effect {
    #[strum(serialize = "0110")]
    _0110,
}

fn spawn_test_effect(mut commands: Commands) {
    commands.spawn((
        Name::new("TestEffect"),
        Effect::_0110,
        Position(Vector2Int::new(3, 3)),
    ));
}
