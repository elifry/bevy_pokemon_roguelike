use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};

use crate::{constants::GAME_SPEED, effects::Effect, map::Position, GameState};

use super::{
    animations::{AnimationFinished, AnimationFrame, Animator},
    assets::EffectAssets,
    get_world_position, EFFECT_Z, FRAME_DURATION_MILLIS,
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_effect_renderer, despawn_effect).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_effect_renderer(
    mut commands: Commands,
    assets: Res<EffectAssets>,
    query: Query<(Entity, &Effect, &Position), Added<Effect>>,
) {
    for (entity, effect, position) in query.iter() {
        let Some(effect_texture_info) = assets
            .0
            .get(effect)
            .and_then(|effect| effect.textures.get("000"))
            .cloned()
        else {
            continue;
        };

        let frames = effect_texture_info
            .frame_order
            .iter()
            .map(|atlas_index| AnimationFrame {
                atlas_index: *atlas_index,
                duration: Duration::from_millis(
                    ((FRAME_DURATION_MILLIS * 2) as f32 / GAME_SPEED).floor() as u64,
                ),
            })
            .collect::<Vec<_>>();

        let v = get_world_position(&position.0, EFFECT_Z);

        let first_index = frames[0].atlas_index;

        commands.entity(entity).insert((
            Animator::new(effect_texture_info.texture_atlas.clone(), frames, false),
            SpriteSheetBundle {
                texture_atlas: effect_texture_info.texture_atlas.clone(),
                transform: Transform::from_translation(v),
                sprite: TextureAtlasSprite {
                    index: first_index,
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn despawn_effect(
    query: Query<(&Animator), With<Effect>>,
    mut ev_animation_finished: EventReader<AnimationFinished>,
    mut commands: Commands,
) {
    for ev in ev_animation_finished.read() {
        let Ok(animator) = query.get(ev.0) else {
            continue;
        };

        if animator.is_finished() {
            commands.entity(ev.0).despawn_recursive();
        }
    }
}
