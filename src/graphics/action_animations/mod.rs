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
        info!("animation finished");
        animation_state.0 = AnimKey::Idle;
    }
}

fn add_action_animation(
    query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (entity, running_action) in query.iter() {
        // ev_animation_playing.send(ActionAnimationPlayingEvent);
        let action = running_action.0.as_any();
        // TODO: move somewhere else
        match action.type_id() {
            id if id == TypeId::of::<SkipAction>() => {
                ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
                ev_animation_next.send(ActionAnimationNextEvent(entity));
            }
            _ => {}
        }
    }
}
