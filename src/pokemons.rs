use bevy::ecs::component::Component;
use strum::{Display, EnumString};

// TODO: Use the enum directly
#[derive(Component, Debug)]
pub struct Pokemon(pub PokemonID);

#[derive(Debug, Hash, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum PokemonID {
    Charmander,
    Rattata,
}
