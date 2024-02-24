use std::time::Duration;

use bevy::prelude::*;
use char_animation::{anim_key::AnimKey, orientation::Orientation, CharAnimation};

use crate::{
    constants::GAME_SPEED,
    graphics::{
        animations::{AnimationFrame, AnimationIndices, Animator},
        FRAME_DURATION_MILLIS,
    },
};

pub fn get_pokemon_animator(
    char_animation_assets: &Assets<CharAnimation>,
    char_animation_handle: &Handle<CharAnimation>,
    anim_key: &AnimKey,
    orientation: &Orientation,
) -> Option<Animator> {
    let char_animation = char_animation_assets
        .get(char_animation_handle)
        .expect("Failed to load char animation for pokemon");

    let animation_data = char_animation
        .anim
        .get(anim_key)
        .expect("Failed to load anim key");

    let is_loop = matches!(anim_key, AnimKey::Idle);

    let animation_indices =
        AnimationIndices::from_animation(orientation, animation_data.durations.len() - 1);

    let frames = animation_data
        .durations
        .iter()
        .enumerate()
        .map(|(index, duration)| AnimationFrame {
            duration: Duration::from_millis(
                ((duration * FRAME_DURATION_MILLIS) as f32 / GAME_SPEED).floor() as u64,
            ),
            atlas_index: animation_indices.first + index,
        })
        .collect::<Vec<_>>();

    Some(Animator::new(
        animation_data.atlas_layout.clone(),
        animation_data.texture.clone(),
        frames,
        is_loop,
        animation_data.return_frame,
        animation_data.hit_frame,
        animation_data.rush_frame,
    ))
}
