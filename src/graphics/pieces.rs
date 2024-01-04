use bevy::{ecs::query, prelude::*};

use crate::{
    map::Position,
    pieces::{Piece, PieceKind},
    GameState,
};

use super::{assets::PokemonAnimationAssets, PIECE_SIZE, PIECE_Z};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_piece_renderer, animate_piece_sprite).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn spawn_piece_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
) {
    for (entity, position, piece) in query.iter() {
        let texture_atlas = match piece.kind {
            PieceKind::Player => assets.files.get("charmander").unwrap().idle.clone(),
            _ => assets.files.get("rattata").unwrap().idle.clone(),
        };

        let animation_indices = AnimationIndices { first: 0, last: 3 };
        let mut sprite = TextureAtlasSprite::new(animation_indices.first);
        sprite.custom_size = Some(Vec2::splat(PIECE_SIZE));
        let v = super::get_world_position(&position, PIECE_Z);

        commands.entity(entity).insert((
            SpriteSheetBundle {
                texture_atlas,
                sprite,
                transform: Transform::from_translation(v),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

fn animate_piece_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
