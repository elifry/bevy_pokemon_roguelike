use bevy::prelude::*;

use crate::{map::Position, pieces::PieceDeathEvent, stats::Health};

use super::{death_action::DeathAction, orient_entity, Action};

#[derive(Debug, Clone)]
pub struct DamageAction {
    pub attacker: Entity,
    pub target: Entity,
    pub value: i32,
    pub move_type: Option<String>,
}

impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        // Get mutable reference to health
        let Some(mut health) = world.get_mut::<Health>(self.target) else {
            return Err(());
        };

        let health_before = health.value;
        health.value = health.value.saturating_sub(self.value);
        let health_after = health.value;
        let actual_damage_dealt = health_before - health_after;

        if let Some(move_type) = &self.move_type {
            info!(
                "Applied {} damage (spell type: {}). Health: {}→{}",
                self.value, move_type, health_before, health_after
            );
        } else {
            info!(
                "Applied {} damage (melee attack). Health: {}→{}",
                self.value, health_before, health_after
            );
        }

        let mut next_actions = vec![];
        if health.is_dead() {
            world.send_event(PieceDeathEvent {
                entity: self.target,
            });
            next_actions.push(Box::new(DeathAction {
                target: self.target,
                attacker: self.attacker,
            }) as Box<dyn Action>);
        }

        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;

        orient_entity(world, self.target, attacker_position.0);

        Ok(next_actions)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }

    fn can_execute(&self, world: &mut World) -> bool {
        return world.get::<Health>(self.target).is_some();
    }
}
