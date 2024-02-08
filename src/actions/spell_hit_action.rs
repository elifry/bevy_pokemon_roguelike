use bevy::prelude::*;

use crate::{spells::SpellHit};

use super::{damage_action::DamageAction, Action};

#[derive(Debug, Clone)]
pub struct SpellHitAction {
    pub caster: Entity,
    pub hit: SpellHit,
    pub target: Entity,
}

impl Action for SpellHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        Ok(vec![Box::new(DamageAction {
            attacker: self.caster,
            target: self.target,
            value: self.hit.damage,
        })])
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }

    fn can_execute(&self, _world: &mut World) -> bool {
        true
    }
}
