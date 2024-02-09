use bevy::prelude::*;

use crate::{
    actions::{spell_hit_action::SpellHitAction, RunningAction},
    effects::Effect,
    graphics::animations::Animator,
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
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();
        let Some(spell_hit_action) = action.downcast_ref::<SpellHitAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands
            .entity(spell_hit_action.target)
            .with_children(|parent| {
                parent.spawn((
                    Name::new(spell_hit_action.hit.visual_effect.to_string()),
                    Effect {
                        name: spell_hit_action.hit.visual_effect.to_string(),
                        is_loop: false,
                    },
                    SpatialBundle {
                        // TODO: target the correct part of the pokemon
                        transform: Transform::from_translation(Vec3::new(0., 15., 0.)),
                        ..default()
                    },
                    AnimationHolder(ActionAnimation::SpellHit(SpellHitAnimation {
                        target: spell_hit_action.target,
                        caster: spell_hit_action.caster,
                    })),
                ));
            });
    }
}

fn spell_hit_animation(
    mut query: Query<(Entity, &mut AnimationHolder, &Animator), With<Effect>>,
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
