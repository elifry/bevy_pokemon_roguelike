use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, transform};

use crate::{
    actions::{melee_hit_action::MeleeHitAction, walk_action::WalkAction, ActionExecutedEvent},
    graphics::animations::Animator,
    map::Position,
    pieces::Piece,
    pokemons::Pokemon,
    GameState,
};

use super::{
    anim_data::{AnimData, AnimInfo, AnimKey},
    animations::{AnimationFrame, AnimationIndices},
    assets::PokemonAnimationAssets,
    get_orientation_from_vector, Orientation, PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE,
};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_pokemon_renderer,
                walk_animation,
                melee_animation,
                path_animator_update,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct PathAnimator {
    pub target: VecDeque<Vec3>,
    pub should_emit_graphics_wait: bool,
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    anim_data_assets: Res<Assets<AnimData>>,
    query: Query<(Entity, &Position, &Pokemon), Added<Pokemon>>,
) {
    for (entity, position, pokemon) in query.iter() {
        let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

        let animator = get_animator(
            &anim_data_assets,
            pokemon_animation,
            AnimKey::Idle,
            Orientation::South,
        );

        let v = super::get_world_position(position, PIECE_Z);
        let sprite = TextureAtlasSprite::new(animator.frames.first().unwrap().atlas_index);
        let texture_atlas = animator.texture_atlas.clone();

        commands.entity(entity).insert((
            animator,
            SpriteSheetBundle {
                texture_atlas,
                sprite,
                transform: Transform::from_translation(v),
                ..default()
            },
        ));
    }
}

fn get_animator(
    anim_data_assets: &Res<'_, Assets<AnimData>>,
    pokemon_animation: &super::assets::PokemonAnimation,
    anim_key: AnimKey,
    orientation: Orientation,
) -> Animator {
    let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
    let anim_info = anim_data.get(anim_key);

    let texture_atlas = match anim_key {
        AnimKey::Walk => pokemon_animation.walk.to_owned(),
        AnimKey::Attack => pokemon_animation.attack.to_owned(),
        AnimKey::Idle => pokemon_animation.idle.to_owned(),
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
        ..default()
    }
}

fn walk_animation(
    mut commands: Commands,
    query_pokemon: Query<(&Pokemon)>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    assets: Res<PokemonAnimationAssets>,
    anim_data_assets: Res<Assets<AnimData>>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let pokemon = query_pokemon.get(action.entity).unwrap();
            let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

            let direction = action.to - action.from;
            let orientation = get_orientation_from_vector(direction);

            let animator = get_animator(
                &anim_data_assets,
                pokemon_animation,
                AnimKey::Walk,
                orientation,
            );

            let target = super::get_world_vec(action.to, PIECE_Z);

            commands.entity(action.entity).insert((
                animator,
                PathAnimator {
                    target: VecDeque::from([target]),
                    should_emit_graphics_wait: false,
                },
            ));
        }
    }
}

fn melee_animation(
    mut commands: Commands,
    query_position: Query<&Position>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeHitAction>() {
            let Ok(base_position) = query_position.get(action.attacker) else {
                continue;
            };
            let base = super::get_world_position(base_position, PIECE_Z);
            let target = 0.5 * (base + super::get_world_vec(action.target, PIECE_Z));
            commands.entity(action.attacker).insert(PathAnimator {
                target: VecDeque::from([target, base]),
                should_emit_graphics_wait: true,
            });
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.target.is_empty() {
            // this entity has completed it's animation
            commands.entity(entity).remove::<PathAnimator>();
            continue;
        }

        if animator.should_emit_graphics_wait {
            ev_wait.send(super::GraphicsWaitEvent);
        }

        let target = *animator.target.front().unwrap();
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_seconds());
        } else {
            // the entity is at the desired path position
            transform.translation = target;
            animator.target.pop_front();
        }
    }
}
