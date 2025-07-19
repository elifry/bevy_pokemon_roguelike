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
            Sprite::default(),
        ));
        entity_commands.insert(VisualEffectImageHandle(effect_texture_info.texture.clone()));
        entity_commands.insert(VisualEffectTextureAtlas(TextureAtlas {
            index: first_index,
            layout: effect_texture_info.layout.clone(),
        }));
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
