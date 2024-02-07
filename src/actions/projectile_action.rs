use bevy::prelude::*;

use crate::{map::Position, pieces::Health, spells::ProjectileSpell, vector2_int::Vector2Int};

use super::{damage_action::DamageAction, Action};

#[derive(Debug, Clone)]
pub struct ProjectileAction {
    pub caster: Entity,
    pub projectile: ProjectileSpell,
    pub target: Vector2Int,
}

impl Action for ProjectileAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        let target_entities = world
            .query_filtered::<(Entity, &Position), With<Health>>()
            .iter(world)
            .filter(|(_, p)| p.0 == self.target)
            .collect::<Vec<_>>();

        if target_entities.is_empty() {
            // return error there if we dont want to play the projectile animation
            //return Err(());
            return Ok(vec![]);
        }

        let result = target_entities
            .iter()
            .map(|target| {
                Box::new(DamageAction {
                    attacker: self.caster,
                    target: target.0,
                    value: self.projectile.damage,
                }) as Box<dyn Action>
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_parallel_execution(&self) -> bool {
        false
    }

    fn can_execute(&self, world: &mut World) -> bool {
        true
    }
}
