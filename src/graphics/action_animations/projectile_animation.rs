use bevy::prelude::*;

use crate::{
    actions::spell_projectile_action::SpellProjectileAction,
    effects::Effect,
    graphics::{
        animations::Animator, get_world_position, EFFECT_Z, POSITION_TOLERANCE, PROJECTILE_SPEED,
    },
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, AnimationHolder,
};

#[derive(Clone)]
pub struct ProjectileAnimation {
    pub caster: Entity,
    pub to: Vec3,
    pub from: Vec3,
    t: f32,
}

pub fn create_projectile_animation(
    action: &SpellProjectileAction,
    from: Vec3,
) -> (Name, Effect, SpatialBundle, AnimationHolder) {
    let to = get_world_position(&action.target, EFFECT_Z);
    (
        Name::new(action.projectile.visual_effect.to_string()),
        Effect {
            name: action.projectile.visual_effect.to_string(),
            is_loop: true,
        },
        SpatialBundle {
            transform: Transform::from_translation(from),
            ..default()
        },
        AnimationHolder(ActionAnimation::Projectile(ProjectileAnimation {
            caster: action.caster,
            to,
            from,
            t: 0.,
        })),
    )
}

pub fn projectile_animation(
    mut query: Query<(Entity, &mut AnimationHolder, &mut Transform, &Animator), With<Effect>>,
    time: Res<Time>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
    mut commands: Commands,
) {
    for (entity, mut animation, mut transform, _animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Projectile(projectile_animation)) = animation.as_mut()
        else {
            continue;
        };

        let d = (projectile_animation.to - transform.translation).length();

        if d > POSITION_TOLERANCE {
            ev_animation_playing.send(ActionAnimationPlayingEvent);

            projectile_animation.t =
                (projectile_animation.t + PROJECTILE_SPEED * time.delta_seconds()).clamp(0., 1.);
            transform.translation = projectile_animation
                .from
                .lerp(projectile_animation.to, projectile_animation.t);
            continue;
        }

        // the entity is at the desired path position
        transform.translation = projectile_animation.to;

        commands.entity(entity).despawn_recursive();

        ev_animation_finished.send(ActionAnimationFinishedEvent(projectile_animation.caster));
        ev_animation_next.send(ActionAnimationNextEvent(projectile_animation.caster));
    }
}
