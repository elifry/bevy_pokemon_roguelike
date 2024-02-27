use bevy::prelude::*;

use self::{
    action_animations::ActionAnimationPlugin, animations::AnimationsPlugin, assets::AssetsPlugin,
    pokemons::PokemonPlugin, tiles::TilesPlugin, ui::UIPlugin, visual_effects::VisualEffectsPlugin,
    world_number::WorldNumberPlugin,
};

pub mod action_animations;
pub mod animations;
pub mod assets;
pub mod pokemons;
pub mod tile_sprite_index;
mod tiles;
pub mod ui;
mod visual_effects;
pub mod world_number;

pub const TILE_Z: f32 = 0.;
pub const TILE_SIZE: f32 = 24.;

pub const POKEMON_Z: f32 = 10.;
pub const EFFECT_Z: f32 = 15.;
pub const SHADOW_POKEMON_Z: f32 = -1.; // relative to `POKEMON_Z`

pub const WALK_SPEED: f32 = 1.43;
pub const PROJECTILE_SPEED: f32 = 1.8;
pub const POSITION_TOLERANCE: f32 = 0.1;

pub const FRAME_DURATION_MILLIS: u32 = 25;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>().add_plugins((
            ActionAnimationPlugin,
            TilesPlugin,
            PokemonPlugin,
            AssetsPlugin,
            AnimationsPlugin,
            VisualEffectsPlugin,
            UIPlugin,
            WorldNumberPlugin,
        ));
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;

pub fn get_world_position(position: &IVec2, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.x as f32,
        TILE_SIZE * position.y as f32,
        z,
    )
}

fn get_world_vec(v: IVec2, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}
