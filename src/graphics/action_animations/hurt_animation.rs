use bevy::prelude::*;

use crate::{
    actions::damage_action::DamageAction,
    graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, AnimationHolder,
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
    mut query: Query<(&mut AnimationHolder, &mut PokemonAnimationState, &Animator)>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Hurt(hurt_animation)) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Hurt {
            animation_state.0 = AnimKey::Hurt;
        }

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(hurt_animation.attacker));
            ev_animation_next.send(ActionAnimationNextEvent(hurt_animation.attacker));
            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
