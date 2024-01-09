use bevy::ecs::component::Component;
use strum::{Display, EnumString};

#[derive(Component)]
pub struct Pokemon(pub Pokemons);

#[derive(Debug, Hash, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Pokemons {
    Charmander,
    Rattata,
}
