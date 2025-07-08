use std::any::TypeId;

use bevy::prelude::*;
use char_animation::anim_key::AnimKey;

use crate::{
    actions::{skip_action::SkipAction, RunningAction},
    GamePlayingSet,
};

use self::{
    attack_animation::AttackAnimationPlugin, death_animation::DeathAnimationPlugin,
    hurt_animation::HurtAnimationPlugin, move_animation::MoveAnimationPlugin,
    projectile_animation::ProjectileAnimationPlugin,
    spell_cast_animation::SpellCastAnimationPlugin, spell_hit_animation::SpellHitAnimationPlugin,
};

use super::pokemons::PokemonAnimationState;

mod attack_animation;
mod death_animation;
mod hurt_animation;
mod move_animation;
mod projectile_animation;
mod spell_cast_animation;
mod spell_hit_animation;

const MAX_ANIMATION_DURATION: f32 = 3.0; // 3 seconds

pub struct ActionAnimationPlugin;

impl Plugin for ActionAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionAnimationPlayingEvent>()
            .add_event::<ActionAnimationFinishedEvent>()
            .add_event::<ActionAnimationNextEvent>()
            .add_plugins((
                AttackAnimationPlugin,
                HurtAnimationPlugin,
                MoveAnimationPlugin,
                ProjectileAnimationPlugin,
                SpellCastAnimationPlugin,
                SpellHitAnimationPlugin,
                DeathAnimationPlugin,
            ))
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
                (check_animation_timeouts).in_set(ActionAnimationSet::PlayAnimations),
            )
            .add_systems(
                Update,
                clean_up_animation.chain().in_set(ActionAnimationSet::Flush),
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

/// Component to track when an action animation started for timeout detection
#[derive(Component)]
pub struct ActionAnimationTimeout {
    pub start_time: f32,
}

impl ActionAnimationTimeout {
    pub fn new(current_time: f32) -> Self {
        Self {
            start_time: current_time,
        }
    }

    pub fn elapsed(&self, current_time: f32) -> f32 {
        current_time - self.start_time
    }

    pub fn is_expired(&self, current_time: f32) -> bool {
        self.elapsed(current_time) > MAX_ANIMATION_DURATION
    }
}

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
    Death(death_animation::DeathAnimation),
}

#[derive(Component)]
struct AnimationHolder(pub ActionAnimation);

/// System to detect and forcibly complete hanging animations
fn check_animation_timeouts(
    time: Res<Time>,
    query: Query<(Entity, &ActionAnimationTimeout), With<RunningAction>>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
    mut commands: Commands,
) {
    let current_time = time.elapsed_seconds();

    for (entity, timeout) in query.iter() {
        if timeout.is_expired(current_time) {
            warn!(
                "Action animation timed out after {:.2} seconds, forcibly completing for entity {:?}",
                timeout.elapsed(current_time),
                entity
            );

            // Forcibly complete the animation
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
            ev_animation_next.send(ActionAnimationNextEvent(entity));

            // Remove the timeout component since we're completing the animation
            commands.entity(entity).remove::<ActionAnimationTimeout>();
        }
    }
}

fn clean_up_animation(
    mut ev_animation_finished: EventReader<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventReader<ActionAnimationNextEvent>,
    mut query_animation_state: Query<&mut PokemonAnimationState>,
    mut commands: Commands,
) {
    for ev in ev_animation_next.read() {
        commands.entity(ev.0).remove::<RunningAction>();
        // Also remove timeout component when animation completes normally
        commands.entity(ev.0).remove::<ActionAnimationTimeout>();
    }
    for ev in ev_animation_finished.read() {
        commands.entity(ev.0).remove::<AnimationHolder>();
        // Remove timeout component when animation holder is cleaned up
        commands.entity(ev.0).remove::<ActionAnimationTimeout>();
        let Ok(mut animation_state) = query_animation_state.get_mut(ev.0) else {
            continue;
        };
        info!("animation finished");
        animation_state.0 = AnimKey::Idle;
    }
}

fn add_action_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    time: Res<Time>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
    mut commands: Commands,
) {
    let current_time = time.elapsed_seconds();

    for (entity, running_action) in query.iter() {
        let action = running_action.0.as_any();

        // Add timeout component to all action animations
        commands
            .entity(entity)
            .insert(ActionAnimationTimeout::new(current_time));

        // Handle special cases like SkipAction
        match action.type_id() {
            id if id == TypeId::of::<SkipAction>() => {
                ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
                ev_animation_next.send(ActionAnimationNextEvent(entity));
            }
            _ => {
                info!(
                    "Started action animation with timeout for entity {:?}",
                    entity
                );
            }
        }
    }
}
