use bevy::prelude::*;

use crate::{
    actions::{damage_action::DamageAction, RunningAction},
    graphics::{anim_data::AnimKey, animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct HurtAnimationPlugin;

impl Plugin for HurtAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_hurt_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (hurt_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Clone)]
pub struct HurtAnimation {
    pub attacker: Entity,
}

fn init_hurt_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(damage_action) = action.downcast_ref::<DamageAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands.entity(damage_action.target).insert((
            AnimationHolder(ActionAnimation::Hurt(HurtAnimation {
                attacker: damage_action.attacker,
            })),
            PokemonAnimationState(AnimKey::Hurt),
        ));
    }
}

fn hurt_animation(
    mut query: Query<(&mut AnimationHolder, &mut PokemonAnimationState, &Animator)>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Hurt(hurt_animation)) = animation.as_mut() else {
            continue;
        };

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(hurt_animation.attacker));
            ev_animation_next.send(ActionAnimationNextEvent(hurt_animation.attacker));
            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
