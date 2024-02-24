use bevy::prelude::*;
use char_animation::anim_key::AnimKey;

use crate::{
    actions::{
        destroy_wall_action::DestroyWallAction, melee_hit_action::MeleeHitAction, RunningAction,
    },
    graphics::{animations::Animator, pokemons::PokemonAnimationState},
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct AttackAnimationPlugin;

impl Plugin for AttackAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_attack_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (attack_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

fn init_attack_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        if action.downcast_ref::<MeleeHitAction>().is_none()
            && action.downcast_ref::<DestroyWallAction>().is_none()
        {
            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands.entity(entity).insert((
            AnimationHolder(ActionAnimation::Attack),
            PokemonAnimationState(AnimKey::Attack),
        ));
    }
}

fn attack_animation(
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
        let AnimationHolder(ActionAnimation::Attack) = animation.as_mut() else {
            continue;
        };

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
            ev_animation_next.send(ActionAnimationNextEvent(entity));
            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
