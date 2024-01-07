use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{Actor, Health, Occupier, Piece, PieceKind},
    vector2_int::Vector2Int,
};

use super::{PossibleActions, AI};

#[derive(Bundle)]
pub struct NPCBundle {
    actor: Actor,
    name: Name,
    health: Health,
    ai: AI,
    possible_actions: PossibleActions,
    occupier: Occupier,
    piece: Piece,
    position: Position,
}

impl Default for NPCBundle {
    fn default() -> Self {
        Self {
            actor: Actor::default(),
            name: Name::new("NPC"),
            health: Health { value: 1 },
            ai: AI,
            possible_actions: PossibleActions::default(),
            occupier: Occupier,
            piece: Piece {
                kind: PieceKind::Npc,
            },
            position: Position(Vector2Int::new(0, 0)),
        }
    }
}

impl NPCBundle {
    pub fn new(name: String, position: Vector2Int) -> Self {
        Self {
            name: Name::new(name),
            position: Position(position),
            ..default()
        }
    }
}
