use std::collections::VecDeque;

use bevy::{prelude::*, transform};

use crate::{
    actions::{melee_hit_action::MeleeHitAction, walk_action::WalkAction, ActionExecutedEvent},
    map::Position,
    pieces::Piece,
    pokemons::Pokemon,
    GameState,
};

use super::{
    anim_data::{AnimData, AnimInfo, AnimKey},
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
                animate_pokemon_sprite,
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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Debug)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    fn new(first: usize, last: usize) -> Self {
        AnimationIndices { first, last }
    }
}

#[derive(Component)]
pub struct AnimationInfo {
    pub indices: AnimationIndices,
    pub orientation: Orientation,
}

impl AnimationInfo {
    fn from_animation(orientation: Orientation, anim_info: &AnimInfo) -> Self {
        let anim_step = anim_info.value().durations.duration.len() - 1;

        let start_index = match orientation {
            Orientation::South => 0,
            Orientation::SouthEst => anim_step + 1,
            Orientation::Est => (anim_step * 2) + 2,
            Orientation::NorthEst => (anim_step * 3) + 3,
            Orientation::North => (anim_step * 4) + 4,
            Orientation::NorthWest => (anim_step * 5) + 5,
            Orientation::West => (anim_step * 6) + 6,
            Orientation::SouthWest => (anim_step * 7) + 7,
        };

        let end_index = start_index + anim_step;

        let indices = AnimationIndices::new(start_index, end_index);

        AnimationInfo {
            orientation,
            indices,
        }
    }
}

fn spawn_pokemon_renderer(
    mut commands: Commands,
    assets: Res<PokemonAnimationAssets>,
    anim_data_assets: Res<Assets<AnimData>>,
    query: Query<(Entity, &Position, &Pokemon), Added<Pokemon>>,
) {
    for (entity, position, pokemon) in query.iter() {
        let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

        let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
        let anim_info = anim_data.get(AnimKey::Idle);

        let texture_atlas = pokemon_animation.idle.to_owned();

        let animation_info = AnimationInfo::from_animation(Orientation::North, &anim_info);

        let mut sprite = TextureAtlasSprite::new(animation_info.indices.first);
        // sprite.custom_size = Some(Vec2::splat(PIECE_SIZE));
        let v = super::get_world_position(position, PIECE_Z);

        info!("Animation indices {:?}", animation_info.indices);

        commands.entity(entity).insert((
            animation_info,
            SpriteSheetBundle {
                texture_atlas,
                sprite,
                transform: Transform::from_translation(v),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

fn animate_pokemon_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationInfo, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (info, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == info.indices.last {
                info.indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn walk_animation(
    mut commands: Commands,
    query_position: Query<(&Pokemon, &Transform)>,
    mut ev_action: EventReader<ActionExecutedEvent>,
    assets: Res<PokemonAnimationAssets>,
    anim_data_assets: Res<Assets<AnimData>>,
) {
    for ev in ev_action.read() {
        let action = ev.0.as_any();
        if let Some(action) = action.downcast_ref::<WalkAction>() {
            let target = super::get_world_vec(action.to, PIECE_Z);

            let (pokemon, transform) = query_position.get(action.entity).unwrap();
            let direction = action.to - action.from;
            let orientation = get_orientation_from_vector(direction);

            let pokemon_animation = assets.0.get(&pokemon.0).unwrap();

            let anim_data = anim_data_assets.get(&pokemon_animation.anim_data).unwrap();
            let anim_info = anim_data.get(AnimKey::Walk);

            let animation_info = AnimationInfo::from_animation(orientation, &anim_info);

            let texture_atlas = pokemon_animation.walk.to_owned();

            let mut sprite = TextureAtlasSprite::new(animation_info.indices.first);
            // sprite.custom_size = Some(Vec2::splat(PIECE_SIZE));

            commands.entity(action.entity).insert((
                animation_info,
                SpriteSheetBundle {
                    texture_atlas,
                    sprite,
                    transform: *transform,
                    ..default()
                },
                PathAnimator {
                    target: VecDeque::from([target]),
                    should_emit_graphics_wait: false,
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
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
