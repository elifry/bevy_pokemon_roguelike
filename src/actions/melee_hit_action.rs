use bevy::prelude::*;

use crate::{ivec2::IVec2Ext, map::Position, stats::Stats};

use super::{damage_action::DamageAction, orient_entity, Action};

#[derive(Debug, Clone)]
pub struct MeleeHitAction {
    pub attacker: Entity,
    pub target: IVec2,
    pub damage: i32,
}

impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Stats>>()
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

    fn can_execute(&self, world: &mut World) -> bool {
        let Some(attacker_position) = world.get::<Position>(self.attacker) else {
            return false;
        };
        if attacker_position.0.manhattan(self.target) > 1 {
            return false;
        };

        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Stats>>()
            .iter(world)
            .filter(|(_, p)| p.0 == self.target)
            .collect::<Vec<_>>();

        if target_entities.is_empty() {
            return false;
        };

        true
    }
}
