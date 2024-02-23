use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Default, Hash, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize, Clone)]
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
