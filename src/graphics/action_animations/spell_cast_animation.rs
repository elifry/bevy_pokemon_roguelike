use bevy::prelude::*;

use crate::{
    actions::spell_action::SpellAction,
    graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, AnimationHolder, GraphicsWaitEvent,
};

#[derive(Clone, Default)]
pub struct SpellCastAnimation {
    pub hit_send: bool,
}

pub fn create_spell_cast_animation(_action: &SpellAction) -> AnimationHolder {
    AnimationHolder(ActionAnimation::SpellCast(SpellCastAnimation {
        hit_send: false,
    }))
}

pub fn spell_cast_animation(
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &Animator,
    )>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_graphics_wait: EventWriter<GraphicsWaitEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::SpellCast(animation)) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Attack {
            animation_state.0 = AnimKey::Attack;
        }

        if animator.is_hit_frame() && !animation.hit_send {
            animation.hit_send = true;
            ev_animation_next.send(ActionAnimationNextEvent(entity));
        }

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));

            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
        ev_graphics_wait.send(GraphicsWaitEvent);
    }
}
