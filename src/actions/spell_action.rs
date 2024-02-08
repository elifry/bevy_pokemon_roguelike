use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{FacingOrientation, Health},
    spells::{Spell, SpellType},
    vector2_int::Vector2Int,
};

use super::{orient_entity, projectile_action::ProjectileAction, Action};

#[derive(Debug, Clone)]
pub struct SpellAction {
    pub caster: Entity,
    pub spell: Spell,
}

impl Action for SpellAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        let world: &mut World = world;

        let Ok((facing_orientation, position)) = world
            .query::<(&FacingOrientation, &Position)>()
            .get(world, self.caster)
        else {
            return Err(());
        };

        let direction_vector = facing_orientation.0.to_vector();
        let position_vector = position.0;

        let direction = direction_vector + position_vector;
        orient_entity(world, self.caster, direction);

        let mut target: Vector2Int = direction_vector * *self.spell.range.end() + position_vector;

        // Looks for any target in range of the spell
        for i in self.spell.range.clone() {
            let test_position = direction_vector * i + position_vector;

            let targetable_entities = world
                .query_filtered::<(Entity, &Position), With<Health>>()
                .iter(world)
                .filter(|(_, p)| p.0 == test_position)
                .collect::<Vec<_>>();

            if targetable_entities.is_empty() {
                continue;
            }
            target = test_position;
            break;
        }

        match &self.spell.spell_type {
            SpellType::Projectile(projectile_spell) => Ok(vec![Box::new(ProjectileAction {
                caster: self.caster,
                projectile: projectile_spell.clone(),
                target,
            })]),
            _ => Ok(vec![]),
        }
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
