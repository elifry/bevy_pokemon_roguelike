use bevy::ecs::component::Component;
use strum::{Display, EnumString};

#[derive(Component, Debug, Hash, PartialEq, Eq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum Pokemon {
    Charmander,
    Rattata,
}
