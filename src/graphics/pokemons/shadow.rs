use std::time::Duration;

use bevy::prelude::*;
use char_animation::CharAnimation;

use crate::{
    constants::GAME_SPEED,
    faction::Faction,
    graphics::{
        animations::{AnimationFrame, AnimationFrameChangedEvent, Animator},
        assets::shadow_assets::ShadowAssets,
        FRAME_DURATION_MILLIS, SHADOW_POKEMON_Z,
    },
    map::{GameMap, Position, TerrainData},
    pieces::FacingOrientation,
};

use super::PokemonAnimationState;

#[derive(Component, Default)]
pub struct PokemonShadow;
impl PokemonShadow {
    pub fn get_animation_frames(
        &self,
        faction: &Faction,
        terrain: &TerrainData,
    ) -> Vec<AnimationFrame> {
        let row = match terrain.r#type {
            crate::map::TerrainType::Ground => 1,
            crate::map::TerrainType::Wall => 0,
            crate::map::TerrainType::Environment(env_type) => match env_type {
                crate::map::EnvironmentType::Water => 3,
                crate::map::EnvironmentType::Lava => 4,
            },
        };
        let faction = match faction {
            Faction::None => 1,
            Faction::Player => 0,
            Faction::Friend => 0,
            Faction::Foe => 2,
        };

        (0..3)
            .map(|i| AnimationFrame {
                atlas_index: row * 3 * 3 + faction * 3 + i,
                duration: Duration::from_millis(
                    ((10 * FRAME_DURATION_MILLIS) as f32 / GAME_SPEED).floor() as u64,
                ),
            })
            .collect::<Vec<_>>()
    }
}

pub fn spawn_shadow_renderer(
    mut commands: Commands,
    shadow_assets: Res<ShadowAssets>,
    map: Res<GameMap>,
    query: Query<(Entity, &PokemonShadow, &Parent), Added<PokemonShadow>>,
    query_parent: Query<(&Position, &Faction)>,
) {
    for (entity, shadow, parent) in query.iter() {
        let Ok((position, faction)) = query_parent.get(**parent) else {
            continue;
        };
        let Some(tile) = map.tiles.get(&position.0) else {
            warn!(
                "Failed to retrieve tile data for {:?} at {:?}",
                entity, position.0,
            );
            continue;
        };

        let frames = shadow.get_animation_frames(faction, tile);

        let atlas = TextureAtlas {
            index: frames[0].atlas_index,
            layout: shadow_assets.atlas_layout.clone(),
        };

        commands.entity(entity).insert((
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., SHADOW_POKEMON_Z)),
                texture: shadow_assets.texture.clone(),
                atlas,
                ..default()
            },
            Animator::new(
                shadow_assets.atlas_layout.clone(),
                shadow_assets.texture.clone(),
                frames,
                true,
                None,
                None,
                None,
            ),
        ));
    }
}

pub fn update_shadow_offsets(
    mut query_parent: Query<(
        &Handle<CharAnimation>,
        &PokemonAnimationState,
        &FacingOrientation,
        &Children,
    )>,
    mut query_shadow: Query<(&Parent, &mut Transform), With<PokemonShadow>>,
    mut ev_frame_changed: EventReader<AnimationFrameChangedEvent>,
    char_animation_assets: Res<Assets<CharAnimation>>,
) {
    for ev in ev_frame_changed.read() {
        let Ok((char_animation_handle, animation_state, orientation, children)) =
            query_parent.get_mut(ev.entity)
        else {
            continue;
        };

        for child in children {
            let Ok(mut shadow) = query_shadow.get_mut(*child) else {
                continue;
            };
            let char_animation = char_animation_assets
                .get(char_animation_handle)
                .expect("Failed to load char animation for pokemon");

            let animation_data = char_animation
                .anim
                .get(&animation_state.0)
                .expect("Failed to load anim key");

            let shadow_offset = animation_data
                .shadow_offsets
                .get(&orientation.0)
                .expect("Failed to get offsets")[ev.frame_index];

            shadow.1.translation = shadow_offset.extend(SHADOW_POKEMON_Z)
        }
    }
}
