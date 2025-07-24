use bevy::prelude::*;

use crate::{
    actions::{spell_hit_action::SpellHitAction, RunningAction},
    graphics::{animations::Animator, pokemons::offsets::PokemonBodyOffset},
    visual_effects::VisualEffect,
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct SpellHitAnimationPlugin;

impl Plugin for SpellHitAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_spell_hit_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (spell_hit_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Clone)]
pub struct SpellHitAnimation {
    pub target: Entity,
    pub caster: Entity,
}

fn init_spell_hit_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    query_children: Query<(&Children)>,
    query_body_offset: Query<Entity, With<PokemonBodyOffset>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(spell_hit_action) = action.downcast_ref::<SpellHitAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        let target_children = query_children.get(spell_hit_action.target).unwrap();

        let target_entity_hist_effect = target_children
            .iter()
            .filter_map(|&child| query_body_offset.get(child).ok())
            .next()
            .unwrap_or(spell_hit_action.target);

        commands
            .entity(target_entity_hist_effect)
            .with_children(|parent| {
                parent.spawn((
                    Name::new(spell_hit_action.hit.visual_effect.to_string()),
                    VisualEffect {
                        name: spell_hit_action.hit.visual_effect,
                        is_loop: false,
                    },
                    Transform::from_translation(Vec3::new(0., 15., 0.)),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    AnimationHolder(ActionAnimation::SpellHit(SpellHitAnimation {
                        target: spell_hit_action.target,
                        caster: spell_hit_action.caster,
                    })),
                ));
            });
    }
}

fn spell_hit_animation(
    mut query: Query<(Entity, &mut AnimationHolder, &Animator), With<VisualEffect>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
    mut commands: Commands,
) {
    for (entity, mut animation, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::SpellHit(animation)) = animation.as_mut() else {
            continue;
        };

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(animation.caster));
            ev_animation_next.send(ActionAnimationNextEvent(animation.caster));
            commands.entity(entity).despawn_recursive();
            continue;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);
    }
}
