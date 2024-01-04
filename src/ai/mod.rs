use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{Actor, Health, Occupier, Piece, PieceKind},
    GameState,
};
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_npcs);
    }
}

fn spawn_npcs(mut commands: Commands) {
    spawn_test_npc(&mut commands, IVec2::new(5, 5));
    spawn_test_npc(&mut commands, IVec2::new(3, 5));
}

fn spawn_test_npc(commands: &mut Commands, position: IVec2) {
    commands.spawn((
        Actor::default(),
        Name::new("NPC"),
        Health { value: 1 },
        Occupier,
        Piece {
            kind: PieceKind::Npc,
        },
        Position(position),
    ));
}
