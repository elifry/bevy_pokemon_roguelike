use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use common::element::Element;
use pokemon_data::PokemonData;

use crate::{
    pokemons::{Pokemon, PokemonMoveset},
    spells::MoveCategory,
    GameState,
};

const MAX_STAT: i32 = 255;
const MAX_HP: i32 = 999;
const DEFAULT_LEVEL: i32 = 50; // Default Pokemon level for damage calculations

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Stats>()
            .register_type::<Health>()
            .register_type::<PokemonTypes>()
            .add_systems(
                Update,
                (
                    update_stats_system,
                    add_health_system,
                    update_moveset_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Debug, Default, Reflect)]
pub struct Stat {
    base: i32,
    bonus: i32,
}

impl Stat {
    pub fn new(base: i32) -> Self {
        Self { base, bonus: 0 }
    }

    pub fn value(&self) -> i32 {
        self.base + self.bonus
    }
}

#[derive(Component, Default, Reflect)]
pub struct Health {
    pub value: i32,
    pub max: i32,
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.value <= 0
    }
}

#[derive(Component, Debug, InspectorOptions, Reflect, Default)]
#[reflect(Component, InspectorOptions)]
pub struct Stats {
    pub health: Stat,
    pub attack: Stat,
    pub special_attack: Stat,
    pub defense: Stat,
    pub special_defense: Stat,
    pub speed: Stat,
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct PokemonTypes {
    pub primary: String,
    pub secondary: Option<String>,
}

impl PokemonTypes {
    pub fn new(element1: String, element2: String) -> Self {
        let secondary = if element2 == "none" {
            None
        } else {
            Some(element2)
        };
        Self {
            primary: element1,
            secondary,
        }
    }

    /// Check if this Pokemon has the given type
    pub fn has_type(&self, move_type: &str) -> bool {
        let has_primary = self.primary == move_type;
        let has_secondary = self.secondary.as_ref().map_or(false, |s| s == move_type);

        info!("Checking STAB: Pokemon types {:?}, move type '{}', primary match: {}, secondary match: {}", 
              self, move_type, has_primary, has_secondary);

        has_primary || has_secondary
    }
}

impl Default for PokemonTypes {
    fn default() -> Self {
        Self {
            primary: "normal".to_string(),
            secondary: None,
        }
    }
}

/// Calculate STAB (Same Type Attack Bonus) multiplier
/// Returns 1.5 if the move type matches any of the Pokemon's types, 1.0 otherwise
pub fn calculate_stab_multiplier(pokemon_types: &PokemonTypes, move_type: &str) -> f32 {
    info!(
        "STAB check: Pokemon has primary='{}', secondary={:?}, move_type='{}'",
        pokemon_types.primary, pokemon_types.secondary, move_type
    );

    let has_type = pokemon_types.has_type(move_type);
    info!("Type match result: {}", has_type);

    if has_type {
        info!("STAB bonus applied: 1.5x");
        1.5
    } else {
        info!("No STAB bonus: 1.0x");
        1.0
    }
}

/// Calculate damage using the Pokemon damage formula
/// Damage = floor(floor(floor(2 * Level / 5 + 2) * AttackStat * MovePower / DefenseStat) / 50 + 2) * Modifiers
pub fn calculate_pokemon_damage(
    attacker_stats: &Stats,
    defender_stats: &Stats,
    move_power: i32,
    move_category: &MoveCategory,
    stab_multiplier: f32,
    level: Option<i32>,
) -> i32 {
    let level = level.unwrap_or(DEFAULT_LEVEL);

    // Select the appropriate attack and defense stats based on move category
    let (attack_stat, defense_stat) = match move_category {
        MoveCategory::Physical => (
            attacker_stats.attack.value(),
            defender_stats.defense.value(),
        ),
        MoveCategory::Special => (
            attacker_stats.special_attack.value(),
            defender_stats.special_defense.value(),
        ),
    };

    info!(
        "Damage calculation: Level={}, Attack={}, Defense={}, MovePower={}, Category={:?}",
        level, attack_stat, defense_stat, move_power, move_category
    );

    // Base damage calculation: ((2 * Level / 5 + 2) * Attack * MovePower / Defense) / 50 + 2
    let level_factor = (2 * level) as f32 / 5.0 + 2.0;
    let base_damage =
        ((level_factor * attack_stat as f32 * move_power as f32) / defense_stat as f32) / 50.0
            + 2.0;

    // Apply modifiers
    let mut final_damage = base_damage * stab_multiplier;

    // Add random factor (0.85 to 1.0)
    use rand::Rng;
    let random_factor = rand::thread_rng().gen_range(0.85..=1.0);
    final_damage *= random_factor;

    // Type effectiveness would go here (1.0 for now)
    let type_effectiveness = 1.0;
    final_damage *= type_effectiveness;

    let final_damage = final_damage.floor() as i32;
    let final_damage = final_damage.max(1); // Minimum 1 damage

    info!(
        "Final damage: {:.1} base * {:.1} STAB * {:.1} random * {:.1} type = {} damage",
        base_damage, stab_multiplier, random_factor, type_effectiveness, final_damage
    );

    final_damage
}

/// Add or update the base stats and types of a pokemon based on its pokemon data
#[allow(clippy::type_complexity)]
fn update_stats_system(
    mut query: Query<
        (
            Entity,
            &Pokemon,
            &Handle<PokemonData>,
            Option<&mut Stats>,
            Option<&mut PokemonTypes>,
        ),
        Changed<Handle<PokemonData>>,
    >,
    pokemon_data: Res<Assets<PokemonData>>,
    mut commands: Commands,
) {
    for (entity, pokemon, pokemon_data_handle, mut stats, mut types) in query.iter_mut() {
        let Some(data) = pokemon_data.get(pokemon_data_handle) else {
            warn!("Unable to retrieve pokemon data for stats");
            continue;
        };

        let pokemon_form = &data.forms[pokemon.form_index];

        let update_base_stats = |stats: &mut Stats| {
            stats.attack.base = pokemon_form.base_atk;
            stats.special_attack.base = pokemon_form.base_m_atk;
            stats.defense.base = pokemon_form.base_def;
            stats.special_defense.base = pokemon_form.base_m_def;
            stats.speed.base = pokemon_form.base_speed;
            stats.health.base = pokemon_form.base_hp;
        };

        if let Some(stats) = stats.as_mut() {
            update_base_stats(stats)
        } else {
            let mut stats = Stats::default();
            update_base_stats(&mut stats);
            commands.entity(entity).insert(stats);
        }

        if let Some(types) = types.as_mut() {
            types.primary = pokemon_form.element1.clone();
            types.secondary = if pokemon_form.element2 == "none" {
                None
            } else {
                Some(pokemon_form.element2.clone())
            };
        } else {
            let pokemon_types =
                PokemonTypes::new(pokemon_form.element1.clone(), pokemon_form.element2.clone());
            commands.entity(entity).insert(pokemon_types);
        }
    }
}

fn add_health_system(mut query: Query<(Entity, &Stats), Added<Stats>>, mut commands: Commands) {
    for (entity, stats) in query.iter_mut() {
        commands.entity(entity).insert(Health {
            value: stats.health.value(),
            max: stats.health.value(),
        });
    }
}

fn update_moveset_system(
    mut query: Query<
        (
            Entity,
            &Pokemon,
            &Handle<PokemonData>,
            Option<&mut PokemonMoveset>,
        ),
        Changed<Handle<PokemonData>>,
    >,
    pokemon_data: Res<Assets<PokemonData>>,
    mut commands: Commands,
) {
    for (entity, pokemon, pokemon_data_handle, mut moveset) in query.iter_mut() {
        let Some(data) = pokemon_data.get(pokemon_data_handle) else {
            warn!("Unable to retrieve pokemon data for moveset");
            continue;
        };

        let pokemon_form = &data.forms[pokemon.form_index];

        // Default starting level for new Pokemon
        const STARTING_LEVEL: i32 = 5;

        if let Some(moveset) = moveset.as_mut() {
            // Update existing moveset
            moveset.moves.clear();
            load_moves_for_level(moveset, pokemon_form, STARTING_LEVEL);
        } else {
            // Create new moveset
            let mut new_moveset = PokemonMoveset::new(STARTING_LEVEL);
            load_moves_for_level(&mut new_moveset, pokemon_form, STARTING_LEVEL);
            commands.entity(entity).insert(new_moveset);
        }
    }
}

fn load_moves_for_level(
    moveset: &mut PokemonMoveset,
    pokemon_form: &pokemon_data::PokemonForm,
    level: i32,
) {
    // Add all moves that can be learned up to the current level
    for level_skill in &pokemon_form.level_skills {
        if level_skill.level <= level {
            moveset.add_move(level_skill.skill.clone());
        }
    }

    // Add some teachable moves for variety (first few teach_skills)
    for (i, teach_skill) in pokemon_form.teach_skills.iter().enumerate() {
        if i < 2 {
            // Only add first 2 teachable moves
            moveset.add_move(teach_skill.skill.clone());
        }
    }

    // Ensure we have at least some moves - add basic moves if none available
    if moveset.moves.is_empty() {
        moveset.add_move("tackle".to_string());
        moveset.add_move("growl".to_string());
    }

    info!(
        "Loaded {} moves for Pokemon: {:?}",
        moveset.moves.len(),
        moveset.moves
    );
}
