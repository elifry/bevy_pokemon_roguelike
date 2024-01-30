use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    actions::{melee_hit_action::MeleeHitAction, walk_action::WalkAction, ActionExecutedEvent},
    graphics::animations::Animator,
    map::Position,
    pieces::{FacingOrientation, Orientation, Piece},
    pokemons::Pokemon,
    GameState,
};

use super::{
    anim_data::{AnimData, AnimKey},
    animations::{AnimationFinished, AnimationFrame, AnimationIndices},
    assets::PokemonAnimationAssets,
    PIECE_Z, POSITION_TOLERANCE,
};

pub struct PokemonPlugin;

impl Plugin for PokemonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                //pokemon_animation_state,
                spawn_pokemon_renderer,
                // fallback_idle_animation,
                update_animator,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Default)]
pub struct PathAnimator {
    pub from: Vec3,
    pub t: f32,
    pub target: VecDeque<Vec3>,
    pub should_emit_graphics_wait: bool,
}

#[derive(Component, Default)]
pub struct PokemonAnimationState(pub AnimKey);

fn fallback_idle_animation(
    mut ev_animation_finished: EventReader<AnimationFinished>,
    mut query_animation_state: Query<&mut PokemonAnimationState>,
) {
    for ev in ev_animation_finished.read() {
        let Ok(mut animation_state) = query_animation_state.get_mut(ev.0) else {
            continue;
        };

        if animation_state.0 != AnimKey::Idle {
            animation_state.0 = AnimKey::Idle;
        }
    }
}

fn update_animator(
    mut query: Query<
        (
            Entity,
            &FacingOrientation,
            &PokemonAnimationState,
            &Pokemon,
            &mut TextureAtlasSprite,
        ),
        Or<(Changed<FacingOrientation>, Changed<PokemonAnimationState>)>,
    >,
    anim_data_assets: Res<Assets<AnimData>>,
    assets: Res<PokemonAnimationAssets>,
    mut commands: Commands,
) {
    for (entity, facing_orientation, animation_state, pokemon, mut sprite) in query.iter_mut() {
        let pokemon_asset = assets.0.get(&pokemon.0).unwrap();
        let animator = get_pokemon_animator(
            &anim_data_assets,
            pokemon_asset,
            &animation_state.0,
            &facing_orientation.0,
        );
        // TODO: Update the texture there
        commands.entity(entity).insert(animator);
    }
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    query: Query<(Entity, &Position, &Pokemon), Added<Pokemon>>,
) {
    for (entity, position, pokemon) in query.iter() {
        let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

        let v = super::get_world_position(&position.0, PIECE_Z);
        let sprite = TextureAtlasSprite {
            index: 0,
            anchor: Anchor::Center,
            ..default()
        };
        let texture_atlas = pokemon_animation.idle.clone();

        commands.entity(entity).insert((
            PokemonAnimationState(AnimKey::Idle),
            SpriteSheetBundle {
                texture_atlas,
                sprite,
                transform: Transform::from_translation(v),
                ..default()
            },
        ));
    }
}

fn get_pokemon_animator(
    anim_data_assets: &Res<'_, Assets<AnimData>>,
    pokemon_animation: &super::assets::PokemonAnimation,
    anim_key: &AnimKey,
    orientation: &Orientation,
) -> Animator {
    let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
    let anim_info = anim_data.get(*anim_key);

    let (texture_atlas, is_loop, emit_graphics_wait) = match anim_key {
        AnimKey::Walk => (pokemon_animation.walk.to_owned(), false, false),
        AnimKey::Attack => (pokemon_animation.attack.to_owned(), false, true),
        AnimKey::Idle => (pokemon_animation.idle.to_owned(), true, false),
        _ => panic!("Not implemented"),
    };

    let animation_indices = AnimationIndices::from_animation(orientation, &anim_info);

    let frames = anim_info
        .value()
        .durations
        .duration
        .iter()
        .enumerate()
        .map(|(index, duration)| AnimationFrame {
            duration: Duration::from_millis((duration.value * 22).try_into().unwrap()),
            atlas_index: animation_indices.first + index,
        })
        .collect::<Vec<_>>();

    Animator {
        texture_atlas: texture_atlas.clone(),
        frames,
        is_loop,
        emit_graphics_wait,
        ..default()
    }
}
