use bevy::prelude::*;

use crate::pieces::Health;

use super::Action;

#[derive(Debug, Clone)]
pub struct DeathAction {
    pub attacker: Entity,
    pub target: Entity,
}

impl Action for DeathAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }

    fn can_execute(&self, world: &mut World) -> bool {
        return world
            .get::<Health>(self.target)
            .map(|health| health.is_dead())
            .unwrap_or(false);
    }
}
