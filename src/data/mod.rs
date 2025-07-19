use bevy::prelude::*;
use pokemon_data::PokemonData;

use crate::{pokemons::Pokemon, GameState};

use self::assets::pokemon_data::PokemonConversion;
use self::assets::{pokemon_data::PokemonDataLookup, DataAssetsPlugin};

/// A wrapper component for Handle<PokemonData> to make it compatible with Bevy 0.15
/// where Handle<T> is no longer automatically a Component.
#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct PokemonDataHandle(pub Handle<PokemonData>);

impl Default for PokemonDataHandle {
    fn default() -> Self {
        Self(Handle::default())
    }
}

pub fn load_pokemon_data(
    query: Query<(Entity, &Pokemon), Without<PokemonDataHandle>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pokemon_conversion: Res<PokemonConversion>,
) {
    for (entity, pokemon) in &query {
        let Some(pokemon_name) = pokemon_conversion.0.get_by_left(&pokemon.id) else {
            warn!("Failed to find pokemon name for ID: {}", pokemon.id);
            continue;
        };
        let file_name = pokemon_name.to_lowercase().replace(' ', "_");
        let pokemon_data_handle = asset_server.load(format!("data/pokemons/{}.ron", file_name));
        commands
            .entity(entity)
            .insert(PokemonDataHandle(pokemon_data_handle));
    }
}

pub mod assets;

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DataAssetsPlugin).add_systems(
            Update,
            update_pokemon_data_handle.run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_pokemon_data_handle(
    query: Query<(Entity, &Pokemon), Changed<Pokemon>>,
    mut commands: Commands,
    pokemon_data_lookup: Res<PokemonDataLookup>,
) {
    for (entity, pokemon) in query.iter() {
        let Some(pokemon_data_handle) = pokemon_data_lookup.0.get(&pokemon.id) else {
            warn!("Failed to find pokemon data handle for ID: {}", pokemon.id);
            continue;
        };
        commands
            .entity(entity)
            .insert(PokemonDataHandle(pokemon_data_handle.clone()));
    }
}
