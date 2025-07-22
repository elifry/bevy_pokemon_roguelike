use bevy::prelude::*;

use crate::{
    actions::{spell_projectile_action::SpellProjectileAction, RunningAction},
    constants::GAME_SPEED,
    graphics::{
        animations::Animator, get_world_position, pokemons::offsets::PokemonHeadOffset, EFFECT_Z,
        POSITION_TOLERANCE, PROJECTILE_SPEED,
    },
    map::Position,
    visual_effects::VisualEffect,
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct ProjectileAnimationPlugin;

impl Plugin for ProjectileAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_projectile_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (projectile_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Component, Clone)]
pub struct ProjectileAnimation {
    pub caster: Entity,
    pub to: Vec3,
    pub from: Vec3,
    t: f32,
}

fn init_projectile_animation(
    query: Query<(Entity, &RunningAction, &Position, &Children), Added<RunningAction>>,
    query_head_offset: Query<&GlobalTransform, With<PokemonHeadOffset>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action, position, children) in query.iter() {
        let action = running_action.0.as_any();
        let Some(spell_projectile_action) = action.downcast_ref::<SpellProjectileAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        let head_offset = children
            .iter()
            .filter_map(|&child| query_head_offset.get(child).ok())
            .next();

        let from = head_offset.map_or(get_world_position(&position.0, EFFECT_Z), |offset| {
            let translation = offset.translation();
            Vec3::new(translation.x, translation.y, EFFECT_Z)
        });
        let to = get_world_position(&spell_projectile_action.target, EFFECT_Z);

        let entity = commands
            .spawn((
                Name::new(spell_projectile_action.projectile.visual_effect.to_string()),
                VisualEffect {
                    name: spell_projectile_action.projectile.visual_effect,
                    is_loop: false,
                },
                Transform::from_translation(from),
                Visibility::default(),
                InheritedVisibility::default(),
            ))
            .id();

        commands.entity(entity).insert((
            ProjectileAnimation {
                caster: spell_projectile_action.caster,
                to,
                from,
                t: 0.,
            },
            AnimationHolder(ActionAnimation::Projectile(ProjectileAnimation {
                caster: spell_projectile_action.caster,
                to,
                from,
                t: 0.,
            })),
        ));
    }
}

fn projectile_animation(
    mut query: Query<(Entity, &mut AnimationHolder, &mut Transform, &Animator), With<VisualEffect>>,
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

            projectile_animation.t = (projectile_animation.t
                + PROJECTILE_SPEED * time.delta_secs() * GAME_SPEED)
                .clamp(0., 1.);
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
