use bevy::{ecs::reflect, prelude::*};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use std::time::Duration;

use crate::pieces::Orientation;
use crate::GameState;

use super::anim_data::AnimInfo;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animation_system.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AnimationFrame {
    pub atlas_index: usize,
    pub duration: Duration,
}

#[derive(Component, Debug, Default, InspectorOptions)]
pub struct Animator {
    pub texture_atlas: Handle<TextureAtlas>,
    pub current_frame: usize,
    pub timer: Timer,
    pub frames: Vec<AnimationFrame>,
}

#[derive(Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        AnimationIndices { first, last }
    }

    pub fn from_animation(orientation: Orientation, anim_info: &AnimInfo) -> Self {
        let anim_step = anim_info.value().durations.duration.len() - 1;

        let start_index = match orientation {
            Orientation::South => 0,
            Orientation::SouthEst => anim_step + 1,
            Orientation::Est => (anim_step * 2) + 2,
            Orientation::NorthEst => (anim_step * 3) + 3,
            Orientation::North => (anim_step * 4) + 4,
            Orientation::NorthWest => (anim_step * 5) + 5,
            Orientation::West => (anim_step * 6) + 6,
            Orientation::SouthWest => (anim_step * 7) + 7,
        };

        let end_index = start_index + anim_step;

        AnimationIndices::new(start_index, end_index)
    }
}

fn animation_system(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Animator,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, mut animator, mut atlas, mut sprite) in &mut query.iter_mut() {
        animator.timer.tick(time.delta());

        if !animator.timer.finished() {
            continue;
        }

        animator.current_frame = if animator.current_frame + 1 < animator.frames.len() {
            animator.current_frame + 1
        } else {
            0
        };

        let Some(frame) = animator.frames.get(animator.current_frame).cloned() else {
            warn!("animation frame not found");
            continue;
        };

        animator.timer.set_duration(frame.duration);
        animator.timer.reset();

        *atlas = animator.texture_atlas.clone();
        sprite.index = frame.atlas_index;
    }
}
