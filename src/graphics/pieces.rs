use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    actions::{walk_action::WalkAction, ActionExecutedEvent},
    map::Position,
    pieces::{Piece, PieceKind},
    GameState,
};

use super::{
    assets::PokemonAnimationAssets, AnimationFinishedEvent, Orientation, PIECE_SIZE, PIECE_SPEED,
    PIECE_Z, POSITION_TOLERANCE,
};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_piece_renderer, animate_piece_sprite).run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, (walk_animation, path_animator_update));
    }
}

#[derive(Component)]
pub struct PathAnimator(pub VecDeque<Vec3>);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    fn new(first: usize, last: usize) -> Self {
        return AnimationIndices { first, last };
    }
}

#[derive(Component)]
pub struct AnimationInfo {
    pub orientation: Orientation,
}

impl AnimationInfo {
    fn indices(&self) -> AnimationIndices {
        match self.orientation {
            Orientation::South => AnimationIndices::new(0, 3),
            Orientation::SouthEst => AnimationIndices::new(4, 7),
            Orientation::Est => AnimationIndices::new(8, 11),
            Orientation::NorthEst => AnimationIndices::new(12, 15),
            Orientation::North => AnimationIndices::new(16, 19),
            Orientation::NorthWest => AnimationIndices::new(20, 23),
            Orientation::West => AnimationIndices::new(24, 27),
            Orientation::SouthWest => AnimationIndices::new(28, 31),
        }
    }
}

fn spawn_piece_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
) {
    for (entity, position, piece) in query.iter() {
        let texture_atlas = match piece.kind {
            PieceKind::Player => assets.0.get("charmander").unwrap().idle.clone(),
            _ => assets.0.get("rattata").unwrap().idle.clone(),
        };

        let animation_indices = AnimationIndices { first: 0, last: 3 };
        let mut sprite = TextureAtlasSprite::new(animation_indices.first);
        sprite.custom_size = Some(Vec2::splat(PIECE_SIZE));
        let v = super::get_world_position(position, PIECE_Z);

        commands.entity(entity).insert((
            SpriteSheetBundle {
                texture_atlas,
                sprite,
                transform: Transform::from_translation(v),
                ..default()
            },
            AnimationInfo {
                orientation: Orientation::North,
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

fn animate_piece_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationInfo, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (info, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == info.indices().last {
                info.indices().first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn walk_animation(
    mut commands: Commands,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let target = super::get_world_vec(action.targeted_position, PIECE_Z);
            commands
                .entity(action.entity)
                .insert(PathAnimator(VecDeque::from([target])));
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
    mut ev_animation_finished: EventWriter<super::AnimationFinishedEvent>,
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.0.is_empty() {
            // this entity has completed it's animation
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }
        ev_wait.send(super::GraphicsWaitEvent);

        let target = *animator.0.front().unwrap();
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_seconds());
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.0.pop_front();
            ev_animation_finished.send(AnimationFinishedEvent);
        }
    }
}
