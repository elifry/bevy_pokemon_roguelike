use bevy::prelude::*;
use char_animation::{anim_key::AnimKey, orientation::Orientation};

use crate::{
    actions::{damage_action::DamageAction, RunningAction},
    graphics::{
        animations::Animator,
        pokemons::{
            offsets::{PokemonBodyOffset, PokemonHeadOffset},
            PokemonAnimationState,
        },
        visual_effects::AutoDespawnEffect,
        world_number::{WorldNumber, WorldNumberType},
    },
    pieces::FacingOrientation,
    visual_effects::VisualEffect,
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
    query_head_offset: Query<Entity, With<PokemonHeadOffset>>,
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

        let target_entity_text_damage =
            query_children
                .get(damage_action.target)
                .map_or(damage_action.target, |children| {
                    children
                        .iter()
                        .find_map(|&child| query_head_offset.get(child).ok())
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
                    VisualEffect {
                        name: "Hit_Neutral",
                        is_loop: false,
                    },
                    AutoDespawnEffect,
                    Transform::default(),
                    Visibility::default(),
                ));
            });

        commands
            .entity(target_entity_text_damage)
            .with_children(|parent| {
                // Text Damage
                parent.spawn((
                    Name::new("Text_Dmg"),
                    WorldNumber {
                        value: -damage_action.value,
                        r#type: WorldNumberType::Damage,
                    },
                    Transform::default(),
                    Visibility::default(),
                ));
            });
    }
}

fn hurt_animation(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationHolder,
        &Animator,
        &mut Transform,
        &FacingOrientation,
    )>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, animator, mut transform, orientation) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Hurt(hurt_animation)) = animation.as_mut() else {
            continue;
        };

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(hurt_animation.attacker));
            ev_animation_next.send(ActionAnimationNextEvent(hurt_animation.attacker));
            continue;
        }

        // Shake the entity
        let shake_value = (time.elapsed_secs() * 40.).cos() * 0.6;
        match orientation.0 {
            // Shake on the Y axis if the actor is oriented on south/north
            Orientation::North | Orientation::South => transform.translation.y += shake_value,
            _ => transform.translation.x += shake_value,
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
