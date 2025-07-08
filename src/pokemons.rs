use bevy::prelude::*;

pub struct PokemonsPlugin;

impl Plugin for PokemonsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pokemon>()
            .register_type::<PokemonMoveset>();
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Pokemon {
    pub id: u32,
    pub form_index: usize,
}

/// Stores the current moves available to this Pokemon
#[derive(Component, Debug, Reflect, Default)]
pub struct PokemonMoveset {
    pub moves: Vec<String>,
    pub level: i32,
}

impl PokemonMoveset {
    pub fn new(level: i32) -> Self {
        Self {
            moves: Vec::new(),
            level,
        }
    }

    pub fn add_move(&mut self, move_name: String) {
        if !self.moves.contains(&move_name) {
            self.moves.push(move_name);
        }
    }

    pub fn get_move(&self, slot: usize) -> Option<&String> {
        self.moves.get(slot)
    }

    pub fn has_move(&self, move_name: &str) -> bool {
        self.moves.iter().any(|m| m == move_name)
    }
}
