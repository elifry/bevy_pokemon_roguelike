use bevy::{prelude::*, sprite::Anchor};

use crate::{
    actions::{
        damage_action::DamageAction, melee_hit_action::MeleeHitAction, skip_action::SkipAction,
        walk_action::WalkAction, ProcessingActionEvent, RunningAction,
    },
    map::CurrentMap,
    vector2_int::Vector2Int,
    GamePlayingSet, GameState,
};

use super::{
    anim_data::AnimKey,
    animations::Animator,
    get_world_position,
    pokemon::{update_animator, PokemonAnimationState},
    POSITION_TOLERANCE, WALK_SPEED,
};

pub struct ActionAnimationPlugin;

impl Plugin for ActionAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionAnimationPlayingEvent>()
            .add_event::<ActionAnimationFinishedEvent>()
            .configure_sets(
                Update,
                (
                    ActionAnimationSet::Prepare,
                    ActionAnimationSet::PlayAnimations,
                    ActionAnimationSet::Flush,
                )
                    .chain()
                    .in_set(GamePlayingSet::Animations),
            )
            .add_systems(
                Update,
                (add_action_animation, update_animator, apply_deferred)
                    .in_set(ActionAnimationSet::Prepare),
            )
            .add_systems(
                Update,
                (move_animation, attack_animation, hurt_animation)
                    .chain()
                    .in_set(ActionAnimationSet::PlayAnimations),
            )
            .add_systems(
                Update,
                (clean_up_animation,)
                    .chain()
                    .in_set(ActionAnimationSet::Flush),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionAnimationSet {
    Prepare,
    PlayAnimations,
    Flush,
}

#[derive(Event, Debug)]
pub struct ActionAnimationPlayingEvent;

#[derive(Event, Debug)]
pub struct ActionAnimationFinishedEvent(pub Entity);

#[derive(Clone)]
pub enum ActionAnimation {
    Move(MoveAnimation),
    Attack,
    Hurt,
    Skip,
}

#[derive(Clone)]
pub struct MoveAnimation {
    pub entity: Entity,
    pub to: Vector2Int,
    pub from: Vector2Int,
    t: f32,
}

impl MoveAnimation {
    pub fn new(entity: Entity, from: Vector2Int, to: Vector2Int) -> Self {
        Self {
            entity,
            from,
            to,
            t: 0.,
        }
    }
}

#[derive(Component)]
pub struct AnimationHolder(pub ActionAnimation);

fn clean_up_animation(
    mut ev_animation_finished: EventReader<ActionAnimationFinishedEvent>,
    mut query_animation_state: Query<&mut PokemonAnimationState>,
    mut commands: Commands,
) {
    for ev in ev_animation_finished.read() {
        commands.entity(ev.0).remove::<AnimationHolder>();
        let Ok(mut animation_state) = query_animation_state.get_mut(ev.0) else {
            continue;
        };
        animation_state.0 = AnimKey::Idle;
    }
}

fn add_action_animation(
    mut query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut commands: Commands,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
) {
    for (entity, running_action) in query.iter_mut() {
        ev_animation_playing.send(ActionAnimationPlayingEvent);

        let action = running_action.0.as_any();

        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let move_animation = AnimationHolder(ActionAnimation::Move(MoveAnimation::new(
                action.entity,
                action.from,
                action.to,
            )));
            commands.entity(entity).insert(move_animation);
        } else if let Some(_action) = action.downcast_ref::<MeleeHitAction>() {
            let attack_animation: AnimationHolder = AnimationHolder(ActionAnimation::Attack);
            commands.entity(entity).insert(attack_animation);
        } else if let Some(action) = action.downcast_ref::<DamageAction>() {
            let attack_animation: AnimationHolder = AnimationHolder(ActionAnimation::Hurt);
            commands.entity(action.target).insert(attack_animation);
        } else {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
        }

        commands.entity(entity).remove::<RunningAction>();
    }
}

pub fn hurt_animation(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &Animator,
    )>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Hurt) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Hurt {
            animation_state.0 = AnimKey::Hurt;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
        }
    }
}

pub fn attack_animation(
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &Animator,
    )>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Attack) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Attack {
            animation_state.0 = AnimKey::Attack;
        }

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        if animator.is_finished() {
            ev_animation_finished.send(ActionAnimationFinishedEvent(entity));
        }
    }
}

pub fn move_animation(
    mut query: Query<(
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &mut Transform,
        &Animator,
    )>,
    time: Res<Time>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
) {
    for (mut animation, mut animation_state, mut transform, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Move(move_animation)) = animation.as_mut() else {
            continue;
        };

        if move_animation.t == 0. {
            animation_state.0 = AnimKey::Walk;
        }

        let target = get_world_position(&move_animation.to, 1.);
        let from = get_world_position(&move_animation.from, 1.);
        let d = (target - transform.translation).length();

        if d > POSITION_TOLERANCE {
            ev_animation_playing.send(ActionAnimationPlayingEvent);
            move_animation.t = (move_animation.t + WALK_SPEED * time.delta_seconds()).clamp(0., 1.);
            transform.translation = from.lerp(target, move_animation.t);
            continue;
        }

        // the entity is at the desired path position
        transform.translation = target;

        if !animator.is_finished() {
            continue;
        }
        ev_animation_finished.send(ActionAnimationFinishedEvent(move_animation.entity));
    }
}
