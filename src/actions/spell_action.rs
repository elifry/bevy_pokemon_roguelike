use bevy::prelude::*;

use crate::{
    map::Position,
    pieces::{FacingOrientation, Health, Orientation, PieceDeathEvent},
    spells::{Spell, SpellType},
    vector2_int::Vector2Int,
};

use super::{orient_entity, Action};

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

        let Ok((facing_orientation, position)) = world
            .query::<(&FacingOrientation, &Position)>()
            .get(world, self.caster)
        else {
            return Err(());
        };

        let range = 3;
        let direction = facing_orientation.0.to_vector() * range + position.0;

        orient_entity(world, self.caster, direction);

        Ok(Vec::new())
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
