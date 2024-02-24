use bevy::prelude::*;
use char_animation::orientation::Orientation;

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
    pub value: i32,
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
