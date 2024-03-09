use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Debug, Default, Reflect, Hash, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize, Clone,
)]
pub enum Orientation {
    #[default]
    South,
    SouthEst,
    Est,
    NorthEst,
    North,
    NorthWest,
    West,
    SouthWest,
}
