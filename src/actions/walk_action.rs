use bevy::prelude::*;

use crate::{
    map::{GameMap, Position},
    pieces::{FacingOrientation, Occupier, Orientation},
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
        // retrieve the board
        let board = world.get_resource::<GameMap>().ok_or(())?;

        // check if the targeted position is on the board
        if !board.tiles.contains_key(&self.to) {
            return Err(());
        };

        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.to)
        {
            warn!("There is already an entity on {:?}", self.to);
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
}
