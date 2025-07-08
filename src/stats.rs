use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use pokemon_data::PokemonData;

use crate::{pokemons::Pokemon, GameState};

const MAX_STAT: i32 = 255;
const MAX_HP: i32 = 999;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Stats>()
            .register_type::<Health>()
            .add_systems(
                Update,
                (update_stats_system, add_health_system)
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

/// Add or update the base stats of a pokemon based on its pokemon data
#[allow(clippy::type_complexity)]
fn update_stats_system(
    mut query: Query<
        (Entity, &Pokemon, &Handle<PokemonData>, Option<&mut Stats>),
        Changed<Handle<PokemonData>>,
    >,
    pokemon_data: Res<Assets<PokemonData>>,
    mut commands: Commands,
) {
    for (entity, pokemon, pokemon_data_handle, mut stats) in query.iter_mut() {
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
