use bevy::prelude::*;
use pokemon_data::PokemonData;

use crate::{pokemons::Pokemon, GameState};

use self::assets::{pokemon_data::PokemonDataLookup, DataAssetsPlugin};

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
        commands.entity(entity).insert(pokemon_data_handle.clone());
    }
}
