use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{Actor, FacingOrientation, Health, Occupier, Orientation, Piece, PieceKind},
    pokemons::Pokemon,
    vector2_int::Vector2Int,
};

use super::{PossibleActions, AI};

#[derive(Bundle)]
pub struct NPCBundle {
    actor: Actor,
    name: Name,
    pokemon: Pokemon,
    health: Health,
    ai: AI,
    possible_actions: PossibleActions,
    occupier: Occupier,
    piece: Piece,
    position: Position,
    facing_orientation: FacingOrientation,
}

impl Default for NPCBundle {
    fn default() -> Self {
        Self {
            actor: Actor::default(),
            name: Name::new("NPC"),
            pokemon: Pokemon::Rattata,
            health: Health { value: 1 },
            ai: AI,
            possible_actions: PossibleActions::default(),
            occupier: Occupier,
            piece: Piece {
                kind: PieceKind::Npc,
            },
            position: Position(Vector2Int::new(0, 0)),
            facing_orientation: FacingOrientation(Orientation::South),
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
