use bevy::prelude::*;

use crate::{
    actions::{damage_action::DamageAction, RunningAction},
    effects::Effect,
    graphics::{
        anim_data::AnimKey,
        animations::Animator,
        effects::AutoDespawnEffect,
        pokemons::{offsets::PokemonBodyOffset, PokemonAnimationState},
    },
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
    query_children: Query<&Children>,
    query_body_offset: Query<Entity, With<PokemonBodyOffset>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(damage_action) = action.downcast_ref::<DamageAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        let target_entity_hurt_effect =
            query_children
                .get(damage_action.target)
                .map_or(damage_action.target, |children| {
                    children
                        .iter()
                        .find_map(|&child| query_body_offset.get(child).ok())
                        .unwrap_or(damage_action.target)
                });

        commands.entity(damage_action.target).insert((
            AnimationHolder(ActionAnimation::Hurt(HurtAnimation {
                attacker: damage_action.attacker,
            })),
            PokemonAnimationState(AnimKey::Hurt),
        ));
        commands
            .entity(target_entity_hurt_effect)
            .with_children(|parent| {
                // Visual Effect
                parent.spawn((
                    Name::new("Hit_Neutral"),
                    Effect {
                        name: "Hit_Neutral",
                        is_loop: false,
                    },
                    AutoDespawnEffect,
                    SpatialBundle::default(),
                ));
            });
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
