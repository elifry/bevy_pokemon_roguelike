use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
}
