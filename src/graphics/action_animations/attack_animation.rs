use bevy::prelude::*;

use crate::graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationPlayingEvent, AnimationHolder,
    GraphicsWaitEvent,
};

pub fn attack_animation(
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &Animator,
    )>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_graphics_wait: EventWriter<GraphicsWaitEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Attack) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Attack {
            animation_state.0 = AnimKey::Attack;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
        ev_graphics_wait.send(GraphicsWaitEvent);

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
        }
    }
}
