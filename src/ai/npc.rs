use bevy::prelude::*;
use char_animation::orientation::Orientation;

use crate::{
    faction::Faction,
    map::Position,
    pieces::{Actor, FacingOrientation, Health, Occupier, Piece, PieceKind},
    pokemons::Pokemon,
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
    faction: Faction,
}

impl Default for NPCBundle {
    fn default() -> Self {
        Self {
            actor: Actor,
            name: Name::new("NPC"),
            pokemon: Pokemon {
                id: 1,
                name: "Bulbasaur".to_string(),
            },
            health: Health { value: 1 },
            ai: AI,
            possible_actions: PossibleActions::default(),
            occupier: Occupier,
            piece: Piece {
                kind: PieceKind::Npc,
            },
            position: Position(IVec2::new(0, 0)),
            facing_orientation: FacingOrientation(Orientation::South),
            faction: Faction::None,
        }
    }
}

impl NPCBundle {
    pub fn new(name: String, position: IVec2, faction: Faction) -> Self {
        Self {
            name: Name::new(name),
            position: Position(position),
            faction,
            ..default()
        }
    }
}
