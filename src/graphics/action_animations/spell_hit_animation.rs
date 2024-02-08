use bevy::prelude::*;

use crate::{
    actions::spell_hit_action::SpellHitAction, effects::Effect, graphics::animations::Animator,
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, AnimationHolder,
};

#[derive(Clone)]
pub struct SpellHitAnimation {
    pub target: Entity,
    pub caster: Entity,
}

pub fn create_spell_hit_animation(
    action: &SpellHitAction,
) -> (Name, Effect, SpatialBundle, AnimationHolder) {
    (
        Name::new(action.hit.visual_effect.to_string()),
        Effect {
            name: action.hit.visual_effect.to_string(),
            is_loop: false,
        },
        SpatialBundle {
            // TODO: target the correct part of the pokemon
            transform: Transform::from_translation(Vec3::new(0., 15., 0.)),
            ..default()
        },
        AnimationHolder(ActionAnimation::SpellHit(SpellHitAnimation {
            target: action.target,
            caster: action.caster,
        })),
    )
}

pub fn spell_hit_animation(
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
