use bevy::prelude::*;

use crate::{
    actions::damage_action::DamageAction,
    graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationPlayingEvent, AnimationHolder,
    GraphicsWaitEvent,
};

#[derive(Clone)]
pub struct HurtAnimation {
    pub attacker: Entity,
}

pub fn create_hurt_animation(action: &DamageAction) -> AnimationHolder {
    AnimationHolder(ActionAnimation::Hurt(HurtAnimation {
        attacker: action.attacker,
    }))
}

pub fn hurt_animation(
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
        let AnimationHolder(ActionAnimation::Hurt(hurt_animation)) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Hurt {
            animation_state.0 = AnimKey::Hurt;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
        ev_graphics_wait.send(GraphicsWaitEvent);

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(hurt_animation.attacker));
        }
    }
}
