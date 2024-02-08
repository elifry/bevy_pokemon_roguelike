use bevy::prelude::*;

use crate::{
    actions::spell_action::SpellAction,
    graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, AnimationHolder,
};

#[derive(Clone, Default)]
pub struct SpellCastAnimation {
    pub hit_send: bool,
    pub cast_animation: AnimKey,
}

pub fn create_spell_cast_animation(action: &SpellAction) -> AnimationHolder {
    AnimationHolder(ActionAnimation::SpellCast(SpellCastAnimation {
        hit_send: false,
        cast_animation: action.spell.cast_animation,
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
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::SpellCast(animation)) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != animation.cast_animation {
            animation_state.0 = animation.cast_animation;
        }

        if animator.is_hit_frame() && !animation.hit_send {
            animation.hit_send = true;
            // TODO: otherwise we can push a component on the entity
            // Then simplify listen when this component is added to emit the next action
            // Remove this component when action animation is finished
            ev_animation_next.send(ActionAnimationNextEvent(entity));
        }

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));

            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
