use bevy::{
    prelude::*,
    render::view::visibility::VisibilityRange,
    sprite::{Sprite, TextureAtlas},
};

use crate::{
    graphics::tile_sprite_index::find_sprite_index_tile,
    map::{GameMap, Position, TerrainType, Tile},
    GameState,
};

use super::{assets::TileAssets, TILE_SIZE, TILE_Z};

use bevy::asset::Assets;
use bevy::sprite::TextureAtlasLayout;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_tile_render.run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_tile_render(
    query: Query<(Entity, &Position, Ref<Tile>)>,
    map: Res<GameMap>,
    mut commands: Commands,
    assets: Res<TileAssets>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    let layout = atlas_layouts
        .get(&assets.tile_layout)
        .expect("Tile atlas layout not loaded");
    for (entity, position, tile) in query.iter() {
        if tile.is_added() || tile.is_changed() {
            let sprite_index = get_tile_map_index(&position.0, &tile.0.r#type, &map);
            let v = super::get_world_position(&position.0, TILE_Z);

            // Ensure sprite_index is within bounds
            let safe_index = if sprite_index < layout.textures.len() {
                sprite_index
            } else {
                warn!(
                    "Sprite index {} out of bounds (max: {}), using default",
                    sprite_index,
                    layout.textures.len() - 1
                );
                0 // Use first texture as fallback
            };

            let urect = layout.textures[safe_index];
            let rect = Rect::new(
                urect.min.x as f32,
                urect.min.y as f32,
                urect.max.x as f32,
                urect.max.y as f32,
            );

            let mut entity_commands = commands.entity(entity);
            entity_commands.insert((
                Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    image: assets.amp_plains_texture.clone(),
                    rect: Some(rect),
                    ..Default::default()
                },
                Transform::from_translation(v),
                Visibility::default(),
                InheritedVisibility::default(),
            ));
        }

        if !tile.is_added() && tile.is_changed() {
            for neighbor in [
                IVec2::new(-1, 0),
                IVec2::new(1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 1),
                IVec2::new(-1, -1),
                IVec2::new(-1, 1),
                IVec2::new(1, -1),
                IVec2::new(1, 1),
            ] {
                let neighbor = position.0 + neighbor;
                if let Some(neighbor_entity) = map.tiles_lookup.get(&neighbor) {
                    if let Ok((_, _, neighbor_tile)) = query.get(*neighbor_entity) {
                        if neighbor_tile.is_changed() {
                            let neighbor_sprite_index =
                                get_tile_map_index(&neighbor, &neighbor_tile.0.r#type, &map);

                            // Ensure neighbor_sprite_index is within bounds
                            let safe_neighbor_index = if neighbor_sprite_index
                                < layout.textures.len()
                            {
                                neighbor_sprite_index
                            } else {
                                warn!("Neighbor sprite index {} out of bounds (max: {}), using default", neighbor_sprite_index, layout.textures.len() - 1);
                                0 // Use first texture as fallback
                            };

                            let neighbor_urect = layout.textures[safe_neighbor_index];
                            let neighbor_rect = Rect::new(
                                neighbor_urect.min.x as f32,
                                neighbor_urect.min.y as f32,
                                neighbor_urect.max.x as f32,
                                neighbor_urect.max.y as f32,
                            );

                            commands.entity(*neighbor_entity).insert(Sprite {
                                custom_size: Some(Vec2::splat(TILE_SIZE)),
                                image: assets.amp_plains_texture.clone(),
                                rect: Some(neighbor_rect),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }
}

fn get_tile_map_index(position: &IVec2, terrain_type: &TerrainType, map: &GameMap) -> usize {
    match terrain_type {
        TerrainType::Ground => find_sprite_index_tile(position, &map.tiles) + 4 * 3,
        TerrainType::Wall => find_sprite_index_tile(position, &map.tiles) + 3,
        TerrainType::Environment(_) => find_sprite_index_tile(position, &map.tiles) + 8 * 3,
    }
}
