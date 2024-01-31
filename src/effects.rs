use strum::{Display, EnumString};

#[derive(Debug, Hash, PartialEq, Eq, EnumString, Display)]
#[strum()]
pub enum Effect {
    _0010,
}
