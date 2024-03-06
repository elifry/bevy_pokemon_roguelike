use bevy::prelude::*;

pub struct PokemonsPlugin;

impl Plugin for PokemonsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pokemon>();
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Pokemon {
    pub id: u32,
    pub form_index: usize,
}
