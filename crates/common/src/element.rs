use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Element {
    None,
    #[serde(rename = "bug")]
    Bug,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "dragon")]
    Dragon,
    #[serde(rename = "electric")]
    Electric,
    #[serde(rename = "fairy")]
    Fairy,
    #[serde(rename = "fighting")]
    Fighting,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "flying")]
    Flying,
    #[serde(rename = "ghost")]
    Ghost,
    #[serde(rename = "grass")]
    Grass,
    #[serde(rename = "ground")]
    Ground,
    #[serde(rename = "ice")]
    Ice,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "poison")]
    Poison,
    #[serde(rename = "psychic")]
    Psychic,
    #[serde(rename = "rock")]
    Rock,
    #[serde(rename = "steel")]
    Steel,
    #[serde(rename = "water")]
    Water,
}
