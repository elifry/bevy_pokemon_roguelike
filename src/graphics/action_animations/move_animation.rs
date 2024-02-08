use bevy::prelude::*;

use crate::{
    actions::walk_action::WalkAction,
    graphics::{
        anim_data::AnimKey, animations::Animator, get_world_position,
        pokemons::PokemonAnimationState, POKEMON_Z, POSITION_TOLERANCE, WALK_SPEED,
    },
    vector2_int::Vector2Int,
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, AnimationHolder, GraphicsWaitEvent,
};

#[derive(Clone)]
pub struct MoveAnimation {
    pub entity: Entity,
    pub to: Vec3,
    pub from: Vec3,
    t: f32,
}

impl MoveAnimation {
    pub fn new(entity: Entity, from: Vector2Int, to: Vector2Int) -> Self {
        Self {
            entity,
            from: get_world_position(&from, POKEMON_Z),
            to: get_world_position(&to, POKEMON_Z),
            t: 0.,
        }
    }
}

pub fn create_move_animation(action: &WalkAction) -> AnimationHolder {
    AnimationHolder(ActionAnimation::Move(MoveAnimation::new(
        action.entity,
        action.from,
        action.to,
    )))
}

pub fn move_animation(
    mut query: Query<(
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &mut Transform,
        &Animator,
    )>,
    time: Res<Time>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_graphics_wait: EventWriter<GraphicsWaitEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, mut animation_state, mut transform, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Move(move_animation)) = animation.as_mut() else {
            continue;
        };

        if move_animation.t == 0. {
            animation_state.0 = AnimKey::Walk;
        }

        let d = (move_animation.to - transform.translation).length();

        if d > POSITION_TOLERANCE {
            ev_animation_playing.send(ActionAnimationPlayingEvent);
            ev_graphics_wait.send(GraphicsWaitEvent);

            move_animation.t = (move_animation.t + WALK_SPEED * time.delta_seconds()).clamp(0., 1.);
            transform.translation = move_animation
                .from
                .lerp(move_animation.to, move_animation.t);
            continue;
        }

        // the entity is at the desired path position
        transform.translation = move_animation.to;

        if !animator.is_finished() {
            continue;
        }

        ev_animation_finished.send(ActionAnimationFinishedEvent(move_animation.entity));
        ev_animation_next.send(ActionAnimationNextEvent(move_animation.entity));
    }
}
