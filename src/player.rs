use bevy::prelude::*;

use crate::map::Position;
use crate::pieces::{Actor, Health, Occupier, Piece, PieceKind};
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Occupier,
        Health { value: 10 },
        Actor::default(),
        Piece {
            kind: PieceKind::Player,
        },
        Position(IVec2::new(0, 0)),
    ));
}
