use bevy::prelude::*;
use std::collections::HashMap;

use crate::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMap::new())
            .add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct GameMap {
    pub tiles: HashMap<IVec2, TileType>,
    pub tiles_lookup: HashMap<IVec2, Entity>,
}

impl GameMap {
    pub fn new() -> Self {
        let mut tiles: HashMap<IVec2, TileType> = HashMap::new();

        for x in 4..20 {
            for y in 1..20 {
                let position = IVec2::new(x, y);
                tiles.insert(position, TileType::Ground);
            }
        }

        for x in 0..11 {
            for y in 0..22 {
                let position = IVec2::new(x, y);
                tiles.entry(position).or_insert(TileType::Wall);
            }
        }

        GameMap {
            tiles,
            tiles_lookup: HashMap::new(),
        }
    }

    pub fn get_neighbors(&self, position: &IVec2) -> HashMap<IVec2, TileType> {
        let mut neighbors: HashMap<IVec2, TileType> = HashMap::new();
        for dy in 0..=2 {
            for dx in 0..=2 {
                let neighbor_position = IVec2 {
                    x: position.x + dx - 1,
                    y: position.y + dy - 1,
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

    pub fn associate_entity_to_tile(&mut self, entity: Entity, position: &IVec2) {
        self.tiles_lookup.insert(*position, entity);
    }
}

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Tilemap;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum TileType {
    Ground,
    Wall,
    Environment, // Water / Lava
}

#[derive(Component, Debug)]
pub struct Tile(pub TileType);

fn spawn_map(mut commands: Commands, mut current_map: ResMut<GameMap>) {
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
        current_map.associate_entity_to_tile(tile, &position);
    }
}
