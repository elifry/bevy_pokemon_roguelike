use bevy::prelude::*;
use char_animation::{file::CharAnimationOffsets, CharAnimation};

use crate::{
    graphics::animations::AnimationFrameChangedEvent, pieces::FacingOrientation, pokemons::Pokemon,
};

use super::PokemonAnimationState;

//
#[derive(Component, Default)]
pub struct PokemonHeadOffset;

#[derive(Component, Default)]
pub struct PokemonBodyOffset;

#[derive(Component, Default)]
pub struct PokemonLeftOffset;

#[derive(Component, Default)]
pub struct PokemonRightOffset;

/// Update the [`PokemonOffsets`] based on its current texture each new animation frame
pub fn update_offsets(
    mut query: Query<(
        &Handle<CharAnimation>,
        &mut CharAnimationOffsets,
        &PokemonAnimationState,
        &FacingOrientation,
    )>,
    mut ev_frame_changed: EventReader<AnimationFrameChangedEvent>,
    char_animation_assets: Res<Assets<CharAnimation>>,
) {
    for ev in ev_frame_changed.read() {
        let Ok((char_animation_handle, mut offsets, animation_state, orientation)) =
            query.get_mut(ev.entity)
        else {
            continue;
        };

        let char_animation = char_animation_assets
            .get(char_animation_handle)
            .expect("Failed to load char animation for pokemon");

        let animation_data = char_animation
            .anim
            .get(&animation_state.0)
            .expect("Failed to load anim key");

        *offsets = animation_data
            .offsets
            .get(&orientation.0)
            .expect("Failed to get offsets")[ev.frame_index]
            .clone();
    }
}

pub fn update_head_offset(
    mut query_head_offset: Query<(&Parent, &mut Transform), With<PokemonHeadOffset>>,
    query_parent: Query<&CharAnimationOffsets, With<Pokemon>>,
) {
    for (parent, mut transform) in query_head_offset.iter_mut() {
        let Ok(offsets) = query_parent.get(parent.get()) else {
            continue;
        };
        transform.translation = Vec3::new(offsets.head.x, offsets.head.y, 0.);
    }
}

pub fn update_body_offset(
    mut query_body_offset: Query<(&Parent, &mut Transform), With<PokemonBodyOffset>>,
    query_parent: Query<&CharAnimationOffsets, With<Pokemon>>,
) {
    for (parent, mut transform) in query_body_offset.iter_mut() {
        let Ok(offsets) = query_parent.get(parent.get()) else {
            continue;
        };
        transform.translation = Vec3::new(offsets.body.x, offsets.body.y, 0.);
    }
}

pub fn debug_offsets(
    query_offsets: Query<(&mut CharAnimationOffsets, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (offsets, global_transform) in query_offsets.iter() {
        // Extract the base translation as Vec2 directly from the global_transform's x and y components
        let base_translation = Vec2::new(
            global_transform.translation().x,
            global_transform.translation().y,
        );

        // Calculate the offset vector based on provided offsets and adjustments
        let body_position = base_translation + offsets.body;
        gizmos.circle_2d(body_position + Vec2::new(0.5, 0.5), 1., Color::GREEN);

        let head_position = base_translation + offsets.head;
        gizmos.circle_2d(head_position + Vec2::new(0.5, 0.5), 1., Color::BLACK);

        let right_position = base_translation + offsets.right;
        gizmos.circle_2d(right_position + Vec2::new(0.5, 0.5), 1., Color::BLUE);

        let left_position = base_translation + offsets.left;
        gizmos.circle_2d(left_position + Vec2::new(0.5, 0.5), 1., Color::RED);
    }
}
