use bevy::prelude::*;

use crate::pieces::{Health, PieceDeathEvent};

use super::Action;

#[derive(Debug, Clone)]
pub struct DamageAction(pub Entity, pub u32);

impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(mut health) = world.get_mut::<Health>(self.0) else {
            return Err(());
        };
        health.value = health.value.saturating_sub(self.1);
        Ok(Vec::new())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }
}
