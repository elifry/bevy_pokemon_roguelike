use bevy::prelude::*;
use std::collections::HashMap;

use crate::{vector2_int::Vector2Int, GameState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentMap>()
            .add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentMap {
    pub tiles: HashMap<Vector2Int, Entity>,
}

#[derive(Component)]
pub struct Position(pub Vector2Int);

#[derive(Component)]
pub struct Tilemap;

#[derive(Component)]
pub struct Tile;

fn spawn_map(mut commands: Commands, mut current_map: ResMut<CurrentMap>) {
    current_map.tiles = HashMap::new();
    let tilemap = commands
        .spawn((Tilemap, Name::new("Tilemap"), SpatialBundle { ..default() }))
        .id();
    for x in 0..8 {
        for y in 0..8 {
            let position = Vector2Int::new(x, y);
            let tile = commands
                .spawn((
                    Position(position),
                    Tile,
                    Name::new(format!("Tile (x:{}, y:{})", position.x, position.y)),
                ))
                .id();
            commands.entity(tilemap).add_child(tile);
            current_map.tiles.insert(position, tile);
        }
    }
}
