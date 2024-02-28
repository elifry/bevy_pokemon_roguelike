use bevy::prelude::*;
use char_animation::anim_key::AnimKey;

use crate::{
    actions::{death_action::DeathAction, RunningAction},
    graphics::{animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct DeathAnimationPlugin;

impl Plugin for DeathAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_death_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (death_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Clone)]
pub struct DeathAnimation {
    pub attacker: Entity,
}

fn init_death_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (_entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(death_action) = action.downcast_ref::<DeathAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands.entity(death_action.target).insert((
            AnimationHolder(ActionAnimation::Death(DeathAnimation{attacker: death_action.attacker})),
            PokemonAnimationState(AnimKey::Hurt),
        ));
    }
}

fn death_animation(
    mut query: Query<(&mut AnimationHolder, &Animator)>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Death(death_animation)) = animation.as_mut() else {
            continue;
        };

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(death_animation.attacker));
            ev_animation_next.send(ActionAnimationNextEvent(death_animation.attacker));
            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
