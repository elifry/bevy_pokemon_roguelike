use bevy::prelude::*;

use crate::{
    map::{GameMap, Position, TileType},
    pieces::Occupier,
    vector2_int::Vector2Int,
};

use super::{orient_entity, Action};

#[derive(Debug, Clone)]
pub struct WalkAction {
    pub entity: Entity,
    pub to: Vector2Int,
    pub from: Vector2Int,
}

#[derive(Event)]
pub struct MovingEvent {
    from: Position,
    to: Position,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        orient_entity(world, self.entity, self.to);

        // get the position of the entity
        let mut position = world.get_mut::<Position>(self.entity).ok_or(())?;
        position.0 = self.to;

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        true
    }

    fn can_execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<GameMap>() else {
            return false;
        };

        // check if the targeted position is on the board
        let Some(tile) = board.tiles.get(&self.to) else {
            return false;
        };

        if *tile != TileType::Ground {
            return false;
        }

        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.to)
        {
            return false;
        };

        true
    }
}
