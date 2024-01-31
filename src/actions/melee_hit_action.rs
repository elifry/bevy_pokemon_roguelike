use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{FacingOrientation, Health, Orientation},
    vector2_int::Vector2Int,
};

use super::{damage_action::DamageAction, orient_entity, Action};

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
            .map(|target| {
                Box::new(DamageAction {
                    attacker: self.attacker,
                    target: target.0,
                    value: self.damage,
                }) as Box<dyn Action>
            })
            .collect::<Vec<_>>();

        orient_entity(world, self.attacker, self.target);

        Ok(result)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }
}
