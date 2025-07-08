use std::time::Duration;

use bevy::prelude::*;

use crate::{constants::GAME_SPEED, visual_effects::VisualEffect, GameState};

use super::{
    animations::{AnimationFinished, AnimationFrame, Animator},
    assets::visual_effect_assets::VisualEffectAssets,
    FRAME_DURATION_MILLIS,
};

pub struct VisualEffectsPlugin;

impl Plugin for VisualEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_visual_effect_renderer, auto_despawn_effect)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct AutoDespawnEffect;

fn spawn_visual_effect_renderer(
    mut commands: Commands,
    visual_effect_assets: Res<VisualEffectAssets>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    query: Query<(Entity, &VisualEffect, &Transform), Added<VisualEffect>>,
) {
    for (entity, effect, _transform) in query.iter() {
        let effect_texture_info =
            if let Some(info) = visual_effect_assets.0.get(effect.name).cloned() {
                info
            } else {
                warn!(
                    "Visual effect texture not found for {}, using fallback",
                    effect.name
                );

                // Create a minimal fallback effect that completes quickly
                // This prevents hanging when visual effects are missing
                let fallback_frames = vec![AnimationFrame {
                    atlas_index: 0,
                    duration: Duration::from_millis(100), // Very short duration
                }];

                commands.entity(entity).insert((
                    AutoDespawnEffect,
                    Animator::new(
                        Default::default(), // atlas_layout
                        Default::default(), // texture
                        fallback_frames,    // frames
                        false,              // is_loop
                        None,               // return_frame
                        None,               // hit_frame
                        None,               // rush_frame
                    ),
                    // Don't add Sprite components for fallback - keep it invisible
                ));
                continue;
            };

        let Some(texture_atlas) = texture_atlases.get(&effect_texture_info.layout) else {
            warn!(
                "Texture atlas not found for {}, using fallback",
                effect.name
            );

            // Same fallback for missing texture atlas
            let fallback_frames = vec![AnimationFrame {
                atlas_index: 0,
                duration: Duration::from_millis(100),
            }];

            commands.entity(entity).insert((
                AutoDespawnEffect,
                Animator::new(
                    Default::default(), // atlas_layout
                    Default::default(), // texture
                    fallback_frames,    // frames
                    false,              // is_loop
                    None,               // return_frame
                    None,               // hit_frame
                    None,               // rush_frame
                ),
            ));
            continue;
        };

        let frames = texture_atlas
            .textures
            .iter()
            .enumerate()
            .map(|(atlas_index, _)| AnimationFrame {
                atlas_index,
                duration: Duration::from_millis(
                    ((FRAME_DURATION_MILLIS * 2) as f32 / GAME_SPEED).floor() as u64,
                ),
            })
            .collect::<Vec<_>>();

        let first_index = frames[0].atlas_index;

        commands.entity(entity).insert((
            Animator::new(
                effect_texture_info.layout.clone(),
                effect_texture_info.texture.clone(),
                frames,
                effect.is_loop,
                None,
                None,
                None,
            ),
            Sprite::default(),
            effect_texture_info.texture.clone(),
            TextureAtlas {
                index: first_index,
                layout: effect_texture_info.layout.clone(),
            },
        ));
    }
}

// TODO: maybe make it more generic to be used with other animator
fn auto_despawn_effect(
    query: Query<&Animator, (With<VisualEffect>, With<AutoDespawnEffect>)>,
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
