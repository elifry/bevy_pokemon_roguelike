use bevy::prelude::*;

use crate::{
    actions::spell_projectile_action::SpellProjectileAction,
    effects::Effect,
    graphics::{
        anim_data::AnimKey, animations::Animator, get_world_position,
        pokemons::PokemonAnimationState, EFFECT_Z, POSITION_TOLERANCE, PROJECTILE_SPEED,
        WALK_SPEED,
    },
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationPlayingEvent, AnimationHolder,
    GraphicsWaitEvent,
};

#[derive(Clone)]
pub struct ProjectileAnimation {
    pub entity: Entity,
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
        },
        SpatialBundle {
            transform: Transform::from_translation(from),
            ..default()
        },
        AnimationHolder(ActionAnimation::Projectile(ProjectileAnimation {
            entity: action.caster,
            to,
            from,
            t: 0.,
        })),
    )
}

pub fn projectile_animation(
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut Effect,
        &mut Transform,
        &Animator,
    )>,
    time: Res<Time>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_graphics_wait: EventWriter<GraphicsWaitEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut commands: Commands,
) {
    for (entity, mut animation, mut effect, mut transform, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Projectile(projectile_animation)) = animation.as_mut()
        else {
            continue;
        };

        let d = (projectile_animation.to - transform.translation).length();

        if d > POSITION_TOLERANCE {
            ev_animation_playing.send(ActionAnimationPlayingEvent);
            ev_graphics_wait.send(GraphicsWaitEvent);

            projectile_animation.t =
                (projectile_animation.t + PROJECTILE_SPEED * time.delta_seconds()).clamp(0., 1.);
            transform.translation = projectile_animation
                .from
                .lerp(projectile_animation.to, projectile_animation.t);
            continue;
        }

        // the entity is at the desired path position
        transform.translation = projectile_animation.to;

        ev_animation_finished.send(ActionAnimationFinishedEvent(projectile_animation.entity));

        commands.entity(entity).despawn_recursive();
    }
}
