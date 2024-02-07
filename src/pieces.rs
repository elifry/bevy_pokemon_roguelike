use bevy::prelude::*;
use strum::{Display, EnumIter};

use crate::vector2_int::Vector2Int;

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
    pub value: usize,
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.value == 0
    }
}

#[derive(Component, Default)]
pub struct Actor;

#[derive(Component)]
pub struct Piece {
    pub kind: PieceKind,
}

pub enum PieceKind {
    Player,
    Npc,
}

#[derive(Component)]
pub struct FacingOrientation(pub Orientation);

#[derive(Debug, Default, EnumIter, Display)]
pub enum Orientation {
    #[default]
    South,
    SouthEst,
    Est,
    NorthEst,
    North,
    NorthWest,
    West,
    SouthWest,
}

impl Orientation {
    pub fn from_vector(direction: Vector2Int) -> Self {
        match direction.normalize() {
            Vector2Int { x: 0, y: -1 } => Orientation::South,
            Vector2Int { x: 1, y: -1 } => Orientation::SouthEst,
            Vector2Int { x: 1, y: 0 } => Orientation::Est,
            Vector2Int { x: 1, y: 1 } => Orientation::NorthEst,
            Vector2Int { x: 0, y: 1 } => Orientation::North,
            Vector2Int { x: -1, y: 1 } => Orientation::NorthWest,
            Vector2Int { x: -1, y: 0 } => Orientation::West,
            Vector2Int { x: -1, y: -1 } => Orientation::SouthWest,
            Vector2Int { x: _, y: _ } => {
                warn!("unable to get orientation from {:?}", direction);
                Orientation::South
            }
        }
    }

    pub fn to_vector(&self) -> Vector2Int {
        match self {
            Orientation::South => Vector2Int { x: 0, y: -1 },
            Orientation::SouthEst => Vector2Int { x: 1, y: -1 },
            Orientation::Est => Vector2Int { x: 1, y: 0 },
            Orientation::NorthEst => Vector2Int { x: 1, y: 1 },
            Orientation::North => Vector2Int { x: 0, y: 1 },
            Orientation::NorthWest => Vector2Int { x: -1, y: 1 },
            Orientation::West => Vector2Int { x: -1, y: 0 },
            Orientation::SouthWest => Vector2Int { x: -1, y: -1 },
        }
    }
}
