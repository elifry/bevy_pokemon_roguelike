use bevy::prelude::*;

use crate::{
    map::{GameMap, Position, TerrainType, Tile},
    GameState,
};

use super::{assets::TileAssets, tile_sprite_index::find_sprite_index_tile, TILE_SIZE, TILE_Z};

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_tile_render)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn update_tile_render(
    query: Query<(Entity, &Position, Ref<Tile>)>,
    mut query_tile: Query<(&Tile, &mut TextureAtlas)>,
    map: Res<GameMap>,
    mut commands: Commands,
    assets: Res<TileAssets>,
) {
    for (entity, position, tile) in query.iter() {
        if tile.is_added() || tile.is_changed() {
            let sprite_index = get_tile_map_index(&position.0, &tile.0, &map);
            let atlas = TextureAtlas {
                index: sprite_index,
                layout: assets.tile_layout.clone(),
            };
            let v = super::get_world_position(&position.0, TILE_Z);

            commands.entity(entity).insert(SpriteSheetBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                atlas,
                texture: assets.amp_plains_texture.clone(),
                transform: Transform::from_translation(v),
                ..Default::default()
            });
        }
        if !tile.is_added() && tile.is_changed() {
            // If a tile is changed we need to update their neighbors
            let neighbors = map.get_neighbors(&position.0);

            for (neighbor, _) in neighbors {
                let Some(neighbor_entity) = map.tiles_lookup.get(&neighbor) else {
                    continue;
                };
                let Ok((neighbor_tile, mut neighbor_sprite)) = query_tile.get_mut(*neighbor_entity)
                else {
                    continue;
                };

                neighbor_sprite.index =
                    get_tile_map_index(&neighbor, &neighbor_tile.0.r#type, &map);
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
