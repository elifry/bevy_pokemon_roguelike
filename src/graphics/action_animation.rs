use bevy::{prelude::*, sprite::Anchor};

use crate::{
    actions::{
        melee_hit_action::MeleeHitAction, skip_action::SkipAction, walk_action::WalkAction,
        ProcessingActionEvent, RunningAction,
    },
    map::CurrentMap,
    vector2_int::Vector2Int,
    GamePlayingSet, GameState,
};

use super::{
    anim_data::AnimKey, animations::Animator, get_world_position, pokemon::PokemonAnimationState,
    POSITION_TOLERANCE, WALK_SPEED,
};

pub struct ActionAnimationPlugin;

impl Plugin for ActionAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationPlayingEvent>().add_systems(
            Update,
            (add_action_animation, move_animation, attack_animation)
                .chain()
                .in_set(GamePlayingSet::Animations),
        );
    }
}

#[derive(Event, Debug)]
pub struct AnimationPlayingEvent;

#[derive(Clone)]
pub enum Animation {
    Move(MoveAnimation),
    Attack,
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
pub struct AnimationHolder(pub Animation);

fn add_action_animation(
    mut query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut commands: Commands,
    mut ev_animation_playing: EventWriter<AnimationPlayingEvent>,
) {
    for (entity, running_action) in query.iter_mut() {
        ev_animation_playing.send(AnimationPlayingEvent);

        let action = running_action.0.as_any();

        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let move_animation = AnimationHolder(Animation::Move(MoveAnimation::new(
                action.entity,
                action.from,
                action.to,
            )));
            commands.entity(entity).insert(move_animation);
        }

        if let Some(_action) = action.downcast_ref::<MeleeHitAction>() {
            let attack_animation: AnimationHolder = AnimationHolder(Animation::Attack);
            commands.entity(entity).insert(attack_animation);
        }

        commands.entity(entity).remove::<RunningAction>();
    }
}

pub fn attack_animation(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &Animator,
    )>,
    mut ev_animation_playing: EventWriter<AnimationPlayingEvent>,
) {
    for (entity, mut animation, mut animation_state, animator) in query.iter_mut() {
        let AnimationHolder(Animation::Attack) = animation.as_mut() else {
            continue;
        };

        if animation_state.0 != AnimKey::Attack {
            animation_state.0 = AnimKey::Attack;
        }

        ev_animation_playing.send(AnimationPlayingEvent);

        if animator.is_finished() {
            // TODO: maybe used event there
            animation_state.0 = AnimKey::Idle;
            commands.entity(entity).remove::<AnimationHolder>();
        }
    }
}

pub fn move_animation(
    mut commands: Commands,
    mut query: Query<(
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &mut Transform,
        &Animator,
    )>,
    time: Res<Time>,
    mut ev_animation_playing: EventWriter<AnimationPlayingEvent>,
) {
    for (mut animation, mut animation_state, mut transform, animator) in query.iter_mut() {
        let AnimationHolder(Animation::Move(move_animation)) = animation.as_mut() else {
            continue;
        };

        if move_animation.t == 0. {
            animation_state.0 = AnimKey::Walk;
        }

        let target = get_world_position(&move_animation.to, 1.);
        let from = get_world_position(&move_animation.from, 1.);
        let d = (target - transform.translation).length();

        if d > POSITION_TOLERANCE {
            ev_animation_playing.send(AnimationPlayingEvent);
            move_animation.t = (move_animation.t + WALK_SPEED * time.delta_seconds()).clamp(0., 1.);
            transform.translation = from.lerp(target, move_animation.t);
            continue;
        }

        // the entity is at the desired path position
        transform.translation = target;

        if !animator.is_finished() {
            continue;
        }

        animation_state.0 = AnimKey::Idle;
        commands
            .entity(move_animation.entity)
            .remove::<AnimationHolder>();
    }
}
