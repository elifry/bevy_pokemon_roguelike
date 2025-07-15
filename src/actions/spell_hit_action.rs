use bevy::prelude::*;

use crate::{
    spells::{MoveCategory, SpellHit},
    stats::{calculate_pokemon_damage, calculate_stab_multiplier, PokemonTypes, Stats},
};

use super::{damage_action::DamageAction, Action};

#[derive(Debug, Clone)]
pub struct SpellHitAction {
    pub caster: Entity,
    pub hit: SpellHit,
    pub target: Entity,
    pub move_type: String,
    pub base_power: i32,
    pub category: MoveCategory,
}

impl Action for SpellHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if !self.can_execute(world) {
            return Err(());
        };

        // Get attacker and defender stats
        let Some(attacker_stats) = world.get::<Stats>(self.caster) else {
            warn!("Attacker has no Stats component");
            return Err(());
        };

        let Some(defender_stats) = world.get::<Stats>(self.target) else {
            warn!("Defender has no Stats component");
            return Err(());
        };

        // Calculate STAB multiplier
        let stab_multiplier = if let Some(attacker_types) = world.get::<PokemonTypes>(self.caster) {
            info!(
                "Spell caster types: primary={}, secondary={:?}",
                attacker_types.primary, attacker_types.secondary
            );
            let multiplier = calculate_stab_multiplier(attacker_types, &self.move_type);
            info!(
                "STAB multiplier for {} spell: {:.1}x",
                self.move_type, multiplier
            );
            if multiplier > 1.0 {
                info!(
                    "STAB applied! {} using {} spell: {:.1}x damage",
                    attacker_types.primary, self.move_type, multiplier
                );
            }
            multiplier
        } else {
            info!("Spell caster has no PokemonTypes component");
            1.0
        };

        // Calculate final damage using Pokemon formula
        let final_damage = calculate_pokemon_damage(
            attacker_stats,
            defender_stats,
            self.base_power,
            &self.category,
            stab_multiplier,
            None, // Use default level for now
        );

        info!(
            "Pokemon damage calculation complete: {} final damage",
            final_damage
        );

        Ok(vec![Box::new(DamageAction {
            attacker: self.caster,
            target: self.target,
            value: self.hit.damage,
            move_type: self.hit.move_type.clone(),
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
