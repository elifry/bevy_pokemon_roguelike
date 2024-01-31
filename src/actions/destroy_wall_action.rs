use bevy::prelude::*;

use crate::{
    map::{GameMap, Position, Tile, TileType},
    pieces::Occupier,
    vector2_int::Vector2Int,
};

use super::{damage_action::DamageAction, orient_entity, walk_action::WalkAction, Action};

#[derive(Clone, Debug)]
pub struct DestroyWallAction {
    pub instigator: Entity,
    pub target: Vector2Int,
}

impl Action for DestroyWallAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        let Some(mut map) = world.get_resource_mut::<GameMap>() else {
            return Err(());
        };

        map.tiles.insert(self.target, TileType::Ground);
        let Some(target_tile) = map.tiles_lookup.get(&self.target).cloned() else {
            return Err(());
        };
        world.entity_mut(target_tile).insert(Tile(TileType::Ground));

        orient_entity(world, self.instigator, self.target);

        let position = world.get_mut::<Position>(self.instigator).ok_or(())?;
        let walk_action = Box::new(WalkAction {
            entity: self.instigator,
            to: self.target,
            from: position.0,
        }) as Box<dyn Action>;

        Ok(vec![walk_action])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }

    fn can_execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<GameMap>() else {
            return false;
        };

        // check if the targeted position is on the board
        let Some(tile) = board.tiles.get(&self.target) else {
            return false;
        };

        // check the tile is wall
        if *tile != TileType::Wall {
            return false;
        }

        // No occupier on the wall
        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.target)
        {
            return false;
        };

        true
    }
}
