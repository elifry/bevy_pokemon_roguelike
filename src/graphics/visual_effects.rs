use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::GAME_SPEED, graphics::assets::visual_effect_assets::VisualEffectAssets,
    visual_effects::VisualEffect, GameState,
};

use super::{
    animations::{AnimationFinished, AnimationFrame, Animator},
    EFFECT_Z, FRAME_DURATION_MILLIS,
};

/// A wrapper component for Handle<Image> to make it compatible with Bevy 0.15
/// where Handle<T> is no longer automatically a Component.
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct VisualEffectImageHandle(pub Handle<Image>);

impl Default for VisualEffectImageHandle {
    fn default() -> Self {
        Self(Handle::default())
    }
}

/// A wrapper component for TextureAtlas to make it compatible with Bevy 0.15
/// where TextureAtlas is no longer automatically a Component.
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct VisualEffectTextureAtlas(pub TextureAtlas);

impl Default for VisualEffectTextureAtlas {
    fn default() -> Self {
        Self(TextureAtlas::default())
    }
}

pub struct VisualEffectsPlugin;

impl Plugin for VisualEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_visual_effect_renderer,
                animate_visual_effects,
                auto_despawn_effect,
            )
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
        let Some(effect_texture_info) = visual_effect_assets.0.get(effect.name).cloned() else {
            warn!("Visual effect texture not found for {}", effect.name);
            continue;
        };

        let Some(texture_atlas) = texture_atlases.get(&effect_texture_info.layout) else {
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
        let first_urect = texture_atlas.textures[first_index];

        // Calculate sprite size from the first frame
        let sprite_size = Vec2::new(
            (first_urect.max.x - first_urect.min.x) as f32,
            (first_urect.max.y - first_urect.min.y) as f32,
        );

        let rect = Rect::new(
            first_urect.min.x as f32,
            first_urect.min.y as f32,
            first_urect.max.x as f32,
            first_urect.max.y as f32,
        );

        let mut entity_commands = commands.entity(entity);
        entity_commands.insert((
            Animator::new(
                effect_texture_info.layout.clone(),
                effect_texture_info.texture.clone(),
                frames,
                effect.is_loop,
                None,
                None,
                None,
            ),
            Sprite {
                custom_size: Some(sprite_size),
                image: effect_texture_info.texture.clone(),
                rect: Some(rect),
                ..Default::default()
            },
            Visibility::default(),
            InheritedVisibility::default(),
        ));
        entity_commands.insert(VisualEffectImageHandle(effect_texture_info.texture.clone()));
    }
}

fn animate_visual_effects(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Animator, &mut Sprite), With<VisualEffect>>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (entity, mut animator, mut sprite) in query.iter_mut() {
        animator.timer.tick(time.delta());

        if !animator.timer.finished() {
            continue;
        }

        if !animator.is_loop && animator.current_frame >= animator.frames.len() - 1 {
            // Animation is finished
            continue;
        }

        let Some(frame) = animator.frames.get(animator.current_frame).cloned() else {
            warn!(
                "animation frame not found for visual effect entity {:?}",
                entity
            );
            continue;
        };

        // Update the sprite to show the current frame
        let layout = atlas_layouts
            .get(&animator.atlas_layout)
            .expect("Visual effect atlas layout not loaded");
        let urect = layout.textures[frame.atlas_index];
        let rect = Rect::new(
            urect.min.x as f32,
            urect.min.y as f32,
            urect.max.x as f32,
            urect.max.y as f32,
        );

        let sprite_size = Vec2::new(
            (urect.max.x - urect.min.x) as f32,
            (urect.max.y - urect.min.y) as f32,
        );

        sprite.rect = Some(rect);
        sprite.custom_size = Some(sprite_size);

        // Set timer for next frame
        animator.timer.set_duration(frame.duration);
        animator.timer.reset();

        // Move to next frame
        animator.current_frame = if animator.current_frame + 1 < animator.frames.len() {
            animator.current_frame + 1
        } else if animator.is_loop {
            0
        } else {
            animator.current_frame + 1
        };
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
