use bevy::prelude::*;

use crate::{map::Position, vector2_int::Vector2Int};

use self::{
    anim_data::{AnimData, AnimDataLoader},
    pieces::PiecesPlugin,
    tiles::TilesPlugin,
};

pub mod anim_data;
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
        app.init_asset::<AnimData>()
            .init_asset_loader::<AnimDataLoader>()
            .add_event::<GraphicsWaitEvent>()
            .add_event::<AnimationFinishedEvent>()
            .add_plugins((TilesPlugin, PiecesPlugin));
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

#[derive(Event)]
pub struct AnimationFinishedEvent;

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

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.0.x as f32,
        TILE_SIZE * position.0.y as f32,
        z,
    )
}

fn get_world_vec(v: Vector2Int, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}
