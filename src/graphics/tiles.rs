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

/// A wrapper component for TextureAtlas to make it compatible with Bevy 0.15
/// where TextureAtlas is no longer automatically a Component.
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct TileTextureAtlas(pub TextureAtlas);

impl Default for TileTextureAtlas {
    fn default() -> Self {
        Self(TextureAtlas::default())
    }
}

/// A wrapper component for Handle<Image> to make it compatible with Bevy 0.15
/// where Handle<T> is no longer automatically a Component.
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct TileImageHandle(pub Handle<Image>);

impl Default for TileImageHandle {
    fn default() -> Self {
        Self(Handle::default())
    }
}

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

fn update_tile_render(
    query: Query<(Entity, &Position, Ref<Tile>)>,
    mut query_tile: Query<(&Tile, &mut TileTextureAtlas)>,
    map: Res<GameMap>,
    mut commands: Commands,
    assets: Res<TileAssets>,
) {
    for (entity, position, tile) in query.iter() {
        if tile.is_added() || tile.is_changed() {
            let sprite_index = find_sprite_index_tile(&position.0, &map.tiles);
            let atlas = TileTextureAtlas(TextureAtlas {
                index: sprite_index,
                layout: assets.tile_layout.clone(),
            });
            let v = super::get_world_position(&position.0, TILE_Z);

            let mut entity_commands = commands.entity(entity);
            entity_commands.insert((
                Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_translation(v),
            ));
            entity_commands.insert(atlas);
            entity_commands.insert(TileImageHandle(assets.amp_plains_texture.clone()));
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
                    if let Ok((neighbor_tile, mut neighbor_sprite)) =
                        query_tile.get_mut(*neighbor_entity)
                    {
                        neighbor_sprite.index = find_sprite_index_tile(&neighbor, &map.tiles);
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
