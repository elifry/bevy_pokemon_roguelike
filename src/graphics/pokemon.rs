use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    actions::{melee_hit_action::MeleeHitAction, walk_action::WalkAction, ActionExecutedEvent},
    graphics::animations::Animator,
    map::Position,
    pieces::{get_orientation_from_vector, Orientation, Piece},
    pokemons::Pokemon,
    GameState,
};

use super::{
    anim_data::{AnimData, AnimKey},
    animations::{AnimationFrame, AnimationIndices},
    assets::PokemonAnimationAssets,
    PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE,
};

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                //pokemon_animation_state,
                spawn_pokemon_renderer,
                path_animator_update,
                walk_animation,
                melee_animation,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, pokemon_animation_state);
    }
}

#[derive(Component)]
pub struct PathAnimator {
    pub target: VecDeque<Vec3>,
    pub should_emit_graphics_wait: bool,
}

#[derive(Component)]
pub struct PokemonAnimationState {
    pub state: AnimKey,
    pub orientation: Orientation,
}

fn pokemon_animation_state(
    mut commands: Commands,
    query: Query<(Entity, &PokemonAnimationState, &Pokemon), Changed<PokemonAnimationState>>,
    anim_data_assets: Res<Assets<AnimData>>,
    assets: Res<PokemonAnimationAssets>,
) {
    for (entity, animation_state, pokemon) in query.iter() {
        info!("pokemon_animation_state {:?}", entity);

        let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

        let animator = get_animator(
            &anim_data_assets,
            pokemon_animation,
            &animation_state.state,
            &animation_state.orientation,
        );

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

        let v = super::get_world_position(position, PIECE_Z);
        let sprite = TextureAtlasSprite {
            index: 0,
            anchor: Anchor::Center,
            ..default()
        };
        let texture_atlas = pokemon_animation.idle.clone();

        commands.entity(entity).insert((
            PokemonAnimationState {
                state: AnimKey::Idle,
                orientation: Orientation::South,
            },
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
    anim_key: &AnimKey,
    orientation: &Orientation,
) -> Animator {
    let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
    let anim_info = anim_data.get(*anim_key);

    let (texture_atlas, is_loop) = match anim_key {
        AnimKey::Walk => (pokemon_animation.walk.to_owned(), false),
        AnimKey::Attack => (pokemon_animation.attack.to_owned(), false),
        AnimKey::Idle => (pokemon_animation.idle.to_owned(), true),
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
        ..default()
    }
}

fn walk_animation(
    mut commands: Commands,
    mut query_animation: Query<&mut PokemonAnimationState>,
    mut ev_action: EventReader<ActionExecutedEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let Ok(mut animation) = query_animation.get_mut(action.entity) else {
                continue;
            };

            let direction = action.to - action.from;

            animation.orientation = get_orientation_from_vector(direction);
            animation.state = AnimKey::Walk;

            let target = super::get_world_vec(action.to, PIECE_Z);

            commands.entity(action.entity).insert((PathAnimator {
                target: VecDeque::from([target]),
                should_emit_graphics_wait: false,
            },));
        }
    }
}

fn melee_animation(
    mut commands: Commands,
    query_position: Query<&Position>,
    mut query_animation: Query<&mut PokemonAnimationState>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<MeleeHitAction>() {
            let Ok(base_position) = query_position.get(action.attacker) else {
                continue;
            };
            let Ok(mut animation) = query_animation.get_mut(action.attacker) else {
                continue;
            };

            let direction = action.target - base_position.0;

            animation.orientation = get_orientation_from_vector(direction);
            animation.state = AnimKey::Attack;

            // let base = super::get_world_position(base_position, PIECE_Z);
            // let target = 0.5 * (base + super::get_world_vec(action.target, PIECE_Z));

            // commands.entity(action.attacker).insert(PathAnimator {
            //     target: VecDeque::from([target, base]),
            //     should_emit_graphics_wait: true,
            // });
            ev_wait.send(super::GraphicsWaitEvent);
        }
    }
}

fn path_animator_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PathAnimator, &mut Transform), With<Pokemon>>,
    mut query_animation: Query<(&mut PokemonAnimationState), With<Pokemon>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    for (entity, mut animator, mut transform) in query.iter_mut() {
        if animator.target.is_empty() {
            // this entity has completed it's animation
            commands.entity(entity).remove::<PathAnimator>();
            if let Ok(mut animation) = query_animation.get_mut(entity) {
                animation.state = AnimKey::Idle;
            }
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
