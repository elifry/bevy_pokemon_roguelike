use bevy::prelude::*;

use crate::actions::Action;

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
}

pub enum PieceKind {
    Player,
    Npc,
}
