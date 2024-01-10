use bevy::prelude::*;
use strum::{Display, EnumIter};

use crate::{actions::Action, vector2_int::Vector2Int};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PieceDeathEvent>();
    }
}

#[derive(Event)]
pub struct PieceDeathEvent {
    pub entity: Entity,
}

#[derive(Component)]
// there can be only a single occupier piece on the same tile
pub struct Occupier;

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component, Default)]
pub struct Actor(pub Vec<Box<dyn Action>>);

#[derive(Component)]
pub struct Piece {
    pub kind: PieceKind,
    pub orientation: Orientation,
}

pub enum PieceKind {
    Player,
    Npc,
}

#[derive(Debug, EnumIter, Display)]
pub enum Orientation {
    South,
    SouthEst,
    Est,
    NorthEst,
    North,
    NorthWest,
    West,
    SouthWest,
}

pub fn get_orientation_from_vector(direction: Vector2Int) -> Orientation {
    match direction {
        Vector2Int { x: 0, y: -1 } => Orientation::South,
        Vector2Int { x: 1, y: -1 } => Orientation::SouthEst,
        Vector2Int { x: 1, y: 0 } => Orientation::Est,
        Vector2Int { x: 1, y: 1 } => Orientation::NorthEst,
        Vector2Int { x: 0, y: 1 } => Orientation::North,
        Vector2Int { x: -1, y: 1 } => Orientation::NorthWest,
        Vector2Int { x: -1, y: 0 } => Orientation::West,
        Vector2Int { x: -1, y: -1 } => Orientation::SouthWest,
        Vector2Int { x, y } => {
            warn!("unable to get orientation from {:?}", direction);
            Orientation::South
        }
    }
}
