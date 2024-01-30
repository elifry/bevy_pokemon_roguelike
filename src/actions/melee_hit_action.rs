use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{FacingOrientation, Health, Orientation},
    vector2_int::Vector2Int,
};

use super::{damage_action::DamageAction, Action};

#[derive(Debug, Clone)]
pub struct MeleeHitAction {
    pub attacker: Entity,
    pub target: Vector2Int,
    pub damage: u32,
}

impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;
        if attacker_position.0.manhattan(self.target) > 1 {
            return Err(());
        };

        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Health>>()
            .iter(world)
            .filter(|(_, p)| p.0 == self.target)
            .collect::<Vec<_>>();

        if target_entities.is_empty() {
            return Err(());
        };

        let result = target_entities
            .iter()
            .map(|e| Box::new(DamageAction(e.0, self.damage)) as Box<dyn Action>)
            .collect::<Vec<_>>();

        let grid_position = world.get::<Position>(self.attacker).ok_or(())?;
        let direction = self.target - grid_position.0;

        let mut facing_orientation = world
            .get_mut::<FacingOrientation>(self.attacker)
            .ok_or(())?;

        facing_orientation.0 = Orientation::from_vector(direction);

        Ok(result)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }
}
