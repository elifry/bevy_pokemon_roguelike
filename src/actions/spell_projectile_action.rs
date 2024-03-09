use bevy::prelude::*;

use crate::{
    map::Position,
    spells::{ProjectileSpell, Spell},
    stats::{Health, Stats},
};

use super::{spell_hit_action::SpellHitAction, Action};

#[derive(Debug, Clone)]
pub struct SpellProjectileAction {
    pub caster: Entity,
    pub spell: Spell,
    pub projectile: ProjectileSpell,
    pub target: IVec2,
}

impl Action for SpellProjectileAction {
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
                Box::new(SpellHitAction {
                    caster: self.caster,
                    target: target.0,
                    hit: self.spell.hit.clone(),
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

    fn can_execute(&self, _world: &mut World) -> bool {
        true
    }
}
