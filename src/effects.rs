use strum::{Display, EnumString};

#[derive(Debug, Hash, PartialEq, Eq, EnumString, Display, Copy, Clone)]
#[strum()]
pub enum EffectID {
    #[strum(serialize = "0110")]
    _0110,
}
