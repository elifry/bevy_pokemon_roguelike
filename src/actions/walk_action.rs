use bevy::prelude::*;

use crate::{
    map::{CurrentMap, Position},
    pieces::Occupier,
    vector2_int::Vector2Int,
};

use super::Action;

pub struct WalkAction {
    pub entity: Entity,
    pub targeted_position: Vector2Int,
}

#[derive(Event)]
pub struct MovingEvent {
    from: Position,
    to: Position,
}

impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        // retrieve the board
        let board = world.get_resource::<CurrentMap>().ok_or(())?;

        // check if the targeted position is on the board
        if !board.tiles.contains_key(&self.targeted_position) {
            return Err(());
        };

        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.0 == self.targeted_position)
        {
            return Err(());
        };

        // get the position of the entity
        let mut position = world.get_mut::<Position>(self.entity).ok_or(())?;
        position.0 = self.targeted_position;

        Ok(Vec::new())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
