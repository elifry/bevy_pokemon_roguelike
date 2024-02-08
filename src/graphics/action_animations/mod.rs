use std::any::TypeId;

use bevy::prelude::*;

use crate::{
    actions::{
        damage_action::DamageAction, destroy_wall_action::DestroyWallAction,
        melee_hit_action::MeleeHitAction, spell_action::SpellAction,
        spell_hit_action::SpellHitAction, spell_projectile_action::SpellProjectileAction,
        walk_action::WalkAction, RunningAction,
    },
    map::Position,
    GamePlayingSet,
};

use super::{
    anim_data::AnimKey, get_world_position, pokemons::PokemonAnimationState, GraphicsWaitEvent,
    EFFECT_Z,
};

mod attack_animation;
mod hurt_animation;
mod move_animation;
mod projectile_animation;
mod spell_cast_animation;
mod spell_hit_animation;

pub struct ActionAnimationPlugin;

impl Plugin for ActionAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionAnimationPlayingEvent>()
            .add_event::<ActionAnimationFinishedEvent>()
            .add_event::<ActionAnimationNextEvent>()
            .configure_sets(
                Update,
                (
                    ActionAnimationSet::Prepare,
                    ActionAnimationSet::Animator,
                    ActionAnimationSet::PlayAnimations,
                    ActionAnimationSet::Flush,
                )
                    .chain()
                    .in_set(GamePlayingSet::Animations),
            )
            .add_systems(
                Update,
                (add_action_animation).in_set(ActionAnimationSet::Prepare),
            )
            .add_systems(
                Update,
                (
                    move_animation::move_animation,
                    attack_animation::attack_animation,
                    hurt_animation::hurt_animation,
                    projectile_animation::projectile_animation,
                    spell_hit_animation::spell_hit_animation,
                    spell_cast_animation::spell_cast_animation,
                )
                    .in_set(ActionAnimationSet::PlayAnimations),
            )
            .add_systems(
                Update,
                (clean_up_animation)
                    .chain()
                    .in_set(ActionAnimationSet::Flush),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ActionAnimationSet {
    Prepare,
    Animator,
    PlayAnimations,
    Flush,
}

#[derive(Event, Debug)]
pub struct ActionAnimationPlayingEvent;

#[derive(Event, Debug)]
pub struct ActionAnimationNextEvent(pub Entity);

#[derive(Event, Debug)]
pub struct ActionAnimationFinishedEvent(pub Entity);

#[derive(Clone)]
pub enum ActionAnimation {
    /* #region spells */
    Projectile(projectile_animation::ProjectileAnimation),
    SpellHit(spell_hit_animation::SpellHitAnimation),
    SpellCast(spell_cast_animation::SpellCastAnimation),
    /* #endregion */
    Move(move_animation::MoveAnimation),
    Attack,
    Hurt(hurt_animation::HurtAnimation),
    Skip,
}

#[derive(Component)]
pub struct AnimationHolder(pub ActionAnimation);

fn clean_up_animation(
    mut ev_animation_finished: EventReader<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventReader<ActionAnimationNextEvent>,
    mut query_animation_state: Query<&mut PokemonAnimationState>,
    mut commands: Commands,
) {
    for ev in ev_animation_next.read() {
        commands.entity(ev.0).remove::<RunningAction>();
    }
    for ev in ev_animation_finished.read() {
        commands.entity(ev.0).remove::<AnimationHolder>();
        let Ok(mut animation_state) = query_animation_state.get_mut(ev.0) else {
            continue;
        };
        animation_state.0 = AnimKey::Idle;
    }
}

fn add_action_animation(
    mut query: Query<(Entity, &Position, &RunningAction), Added<RunningAction>>,
    mut commands: Commands,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
    mut ev_graphics_wait: EventWriter<GraphicsWaitEvent>,
) {
    for (entity, position, running_action) in query.iter_mut() {
        ev_animation_playing.send(ActionAnimationPlayingEvent);
        ev_graphics_wait.send(GraphicsWaitEvent);

        let action = running_action.0.as_any();

        match action.type_id() {
            id if id == TypeId::of::<WalkAction>() => {
                let action = action.downcast_ref::<WalkAction>().unwrap();
                commands
                    .entity(entity)
                    .insert(move_animation::create_move_animation(action));
            }
            id if id == TypeId::of::<MeleeHitAction>() => {
                commands
                    .entity(entity)
                    .insert(AnimationHolder(ActionAnimation::Attack));
            }
            id if id == TypeId::of::<DamageAction>() => {
                let action = action.downcast_ref::<DamageAction>().unwrap();
                commands
                    .entity(action.target)
                    .insert(hurt_animation::create_hurt_animation(action));
            }
            id if id == TypeId::of::<DestroyWallAction>() || id == TypeId::of::<SpellAction>() => {
                let action = action.downcast_ref::<SpellAction>().unwrap();
                commands
                    .entity(entity)
                    .insert(spell_cast_animation::create_spell_cast_animation(action));
            }
            id if id == TypeId::of::<SpellProjectileAction>() => {
                let action = action.downcast_ref::<SpellProjectileAction>().unwrap();
                let from = get_world_position(&position.0, EFFECT_Z);
                commands.spawn(projectile_animation::create_projectile_animation(
                    action, from,
                ));
            }
            id if id == TypeId::of::<SpellHitAction>() => {
                let action = action.downcast_ref::<SpellHitAction>().unwrap();

                commands.entity(action.target).with_children(|parent| {
                    parent.spawn(spell_hit_animation::create_spell_hit_animation(action));
                });
            }
            _ => {
                ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
                ev_animation_next.send(ActionAnimationNextEvent(entity));
            }
        }
    }
}
