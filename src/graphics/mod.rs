use bevy::prelude::*;

use crate::map::Position;

use self::{pieces::PiecesPlugin, tiles::TilesPlugin};

pub mod assets;
mod pieces;
mod tiles;

pub const TILE_Z: f32 = 0.;
pub const TILE_SIZE: f32 = 24.;

pub const PIECE_Z: f32 = 10.;
pub const PIECE_SIZE: f32 = 32.;
pub const PIECE_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TilesPlugin, PiecesPlugin));
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.0.x as f32,
        TILE_SIZE * position.0.y as f32,
        z,
    )
}

fn get_world_vec(v: IVec2, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}
