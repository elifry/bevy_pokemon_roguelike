use bevy::prelude::*;
use char_animation::orientation::Orientation;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FacingOrientation>()
            .add_event::<PieceDeathEvent>();
    }
}

#[derive(Event)]
pub struct PieceDeathEvent {
    pub entity: Entity,
}

#[derive(Component, Debug)]
// there can be only a single occupier piece on the same tile
pub struct Occupier;

#[derive(Component, Default, Debug)]
pub struct Actor;

#[derive(Component)]
pub struct Piece {
    pub kind: PieceKind,
}

pub enum PieceKind {
    Player,
    Npc,
}

#[derive(Component, Debug, Reflect)]
pub struct FacingOrientation(pub Orientation);
