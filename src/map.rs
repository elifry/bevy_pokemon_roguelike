use bevy::prelude::*;
use std::collections::HashMap;

use crate::{vector2_int::Vector2Int, GameState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMap::new())
            .add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct GameMap {
    pub tiles: HashMap<Vector2Int, TileType>,
    pub tiles_lookup: HashMap<Vector2Int, Entity>,
}

impl GameMap {
    pub fn new() -> Self {
        let mut tiles: HashMap<Vector2Int, TileType> = HashMap::new();

        for x in 4..8 {
            for y in 1..8 {
                let position = Vector2Int::new(x, y);
                tiles.insert(position, TileType::Ground);
            }
        }

        for x in 0..11 {
            for y in 0..11 {
                let position = Vector2Int::new(x, y);
                tiles.entry(position).or_insert(TileType::Wall);
            }
        }

        GameMap {
            tiles,
            tiles_lookup: HashMap::new(),
        }
    }

    pub fn get_neighbors(&self, position: &Vector2Int) -> HashMap<Vector2Int, TileType> {
        let mut neighbors: HashMap<Vector2Int, TileType> = HashMap::new();
        for dy in 0..=2 {
            for dx in 0..=2 {
                let neighbor_position = Vector2Int {
                    x: position.x + dx as i32 - 1,
                    y: position.y + dy as i32 - 1,
                };
                if neighbor_position == *position {
                    continue;
                }
                if self.tiles.contains_key(&neighbor_position) {
                    neighbors.insert(neighbor_position, self.tiles[&neighbor_position]);
                }
            }
        }
        neighbors
    }

    pub fn associate_entity_to_tile(&mut self, entity: Entity, position: &Vector2Int) {
        self.tiles_lookup.insert(*position, entity);
    }
}

#[derive(Component)]
pub struct Position(pub Vector2Int);

#[derive(Component)]
pub struct Tilemap;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum TileType {
    Ground,
    Wall,
    Environment, // Water / Lava
}

#[derive(Component)]
pub struct Tile(pub TileType);

fn spawn_map(mut commands: Commands, current_map: Res<GameMap>) {
    let tilemap = commands
        .spawn((Tilemap, Name::new("Tilemap"), SpatialBundle { ..default() }))
        .id();

    for (position, tile_type) in current_map.tiles.clone().into_iter() {
        let tile = commands
            .spawn((
                Position(position),
                Tile(tile_type),
                Name::new(format!("Tile (x:{}, y:{})", position.x, position.y)),
            ))
            .id();
        commands.entity(tilemap).add_child(tile);
    }
}
