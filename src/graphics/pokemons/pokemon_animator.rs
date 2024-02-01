use std::time::Duration;

use bevy::{prelude::*};

use crate::{
    constants::GAME_SPEED,
    graphics::{
        anim_data::{AnimData, AnimKey},
        animations::{AnimationFrame, AnimationIndices, Animator},
        assets::{AnimTextureType, PokemonAnimation},
    },
    pieces::{Orientation},
};



pub fn get_pokemon_animator(
    anim_data_assets: &Res<'_, Assets<AnimData>>,
    pokemon_animation: &PokemonAnimation,
    anim_key: &AnimKey,
    anim_texture_type: &AnimTextureType,
    orientation: &Orientation,
) -> Option<Animator> {
    let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
    let anim_info = anim_data.get(*anim_key);

    let is_loop = matches!(anim_key, AnimKey::Idle);

    let animation_indices = AnimationIndices::from_animation(orientation, &anim_info);

    let frames = anim_info
        .value()
        .durations
        .duration
        .iter()
        .enumerate()
        .map(|(index, duration)| AnimationFrame {
            duration: Duration::from_millis(
                ((duration.value * 22) as f32 / GAME_SPEED).floor() as u64
            ),
            atlas_index: animation_indices.first + index,
        })
        .collect::<Vec<_>>();

    let Some(texture_atlas) = pokemon_animation
        .textures
        .get(anim_key)
        .and_then(|t| t.get(anim_texture_type))
    else {
        return None;
    };

    Some(Animator::new(texture_atlas.clone(), frames, is_loop))
}
