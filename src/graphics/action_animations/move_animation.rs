use bevy::prelude::*;

use crate::{
    actions::{walk_action::WalkAction, RunningAction},
    constants::GAME_SPEED,
    graphics::{
        anim_data::AnimKey, animations::Animator, get_world_position,
        pokemons::PokemonAnimationState, POKEMON_Z, POSITION_TOLERANCE, WALK_SPEED,
    },
    map::Position,
    vector2_int::Vector2Int,
};

use super::{
    ActionAnimation, ActionAnimationFinishedEvent, ActionAnimationNextEvent,
    ActionAnimationPlayingEvent, ActionAnimationSet, AnimationHolder,
};

pub struct MoveAnimationPlugin;

impl Plugin for MoveAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (init_move_animation).in_set(ActionAnimationSet::Prepare),
        )
        .add_systems(
            Update,
            (move_animation).in_set(ActionAnimationSet::PlayAnimations),
        );
    }
}

#[derive(Clone)]
pub struct MoveAnimation {
    pub entity: Entity,
    pub to: Vec3,
    pub from: Vec3,
    t: f32,
}

impl MoveAnimation {
    pub fn new(entity: Entity, from: Vector2Int, to: Vector2Int) -> Self {
        Self {
            entity,
            from: get_world_position(&from, POKEMON_Z),
            to: get_world_position(&to, POKEMON_Z),
            t: 0.,
        }
    }
}

fn init_move_animation(
    mut query: Query<(Entity, &RunningAction), Added<RunningAction>>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut commands: Commands,
) {
    for (entity, running_action) in query.iter_mut() {
        let action = running_action.0.as_any();
        let Some(walk_action) = action.downcast_ref::<WalkAction>() else {
            continue;
        };

        ev_animation_playing.send(ActionAnimationPlayingEvent);

        commands.entity(entity).insert((
            AnimationHolder(ActionAnimation::Move(MoveAnimation::new(
                walk_action.entity,
                walk_action.from,
                walk_action.to,
            ))),
            PokemonAnimationState(AnimKey::Walk),
        ));
    }
}

fn move_animation(
    mut query: Query<(
        &mut AnimationHolder,
        &mut PokemonAnimationState,
        &mut Transform,
        &Animator,
    )>,
    time: Res<Time>,
    mut ev_animation_playing: EventWriter<ActionAnimationPlayingEvent>,
    mut ev_animation_finished: EventWriter<ActionAnimationFinishedEvent>,
    mut ev_animation_next: EventWriter<ActionAnimationNextEvent>,
) {
    for (mut animation, mut animation_state, mut transform, animator) in query.iter_mut() {
        let AnimationHolder(ActionAnimation::Move(move_animation)) = animation.as_mut() else {
            continue;
        };

        let d = (move_animation.to - transform.translation).length();

        if d > POSITION_TOLERANCE {
            ev_animation_playing.send(ActionAnimationPlayingEvent);

            move_animation.t =
                (move_animation.t + WALK_SPEED * time.delta_seconds() * GAME_SPEED).clamp(0., 1.);
            transform.translation = move_animation
                .from
                .lerp(move_animation.to, move_animation.t);
            continue;
        }

        // the entity is at the desired path position
        // transform.translation = move_animation.to;

        ev_animation_next.send(ActionAnimationNextEvent(move_animation.entity));

        if !animator.is_finished() {
            continue;
        }

        ev_animation_finished.send(ActionAnimationFinishedEvent(move_animation.entity));
    }
}
