use bevy::prelude::*;

#[derive(Debug, Component)]
pub enum Faction {
    None,
    Player,
    Friend,
    Foe,
}
