use strum::{Display, EnumIter};

#[derive(Debug, Default, EnumIter, Display)]
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
