use std::{
    fs::File,
    io::{self, Write},
};

use bevy::{asset::Asset, reflect::TypePath};
use common::{element::Element, map_status::MapStatus};
use serde::{Deserialize, Serialize};

#[derive(TypePath, Asset, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonData {
    pub name: TextData,
    pub released: bool,
    pub comment: String,
    pub title: TextData,
    pub index_num: i64,
    pub exp_table: String,
    pub skill_group1: String,
    pub skill_group2: String,
    pub join_rate: i64,
    pub promote_from: String,
    pub promotions: Vec<Promotion>,
    pub forms: Vec<PokemonForm>,
}

impl PokemonData {
    pub fn load(buffer: &[u8]) -> Result<Self, ron::Error> {
        let pokemon_data = ron::de::from_bytes(buffer)?;
        Ok(pokemon_data)
    }

    pub fn save(&self, file: &mut File) -> Result<(), io::Error> {
        let buffer = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();
        file.write_all(buffer.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonForm {
    pub released: bool,
    pub generation: i64,
    pub genderless_weight: i64,
    pub male_weight: i64,
    pub female_weight: i64,
    pub base_hp: i64,
    pub base_atk: i64,
    pub base_def: i64,
    pub base_m_atk: i64,
    pub base_m_def: i64,
    pub base_speed: i64,
    pub exp_yield: i64,
    pub height: f64,
    pub weight: f64,
    pub personalities: Vec<i64>,
    pub teach_skills: Vec<PokemonSkill>,
    pub shared_skills: Vec<PokemonSkill>,
    pub secret_skills: Vec<PokemonSkill>,
    pub form_name: TextData,
    pub temporary: bool,
    pub promote_form: i64,
    pub element1: String,
    pub element2: String,
    pub intrinsic1: String,
    pub intrinsic2: String,
    pub intrinsic3: String,
    pub level_skills: Vec<PokemonLevelSkill>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextData {
    pub default_text: String,
    pub local_texts: LocalTexts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalTexts {
    pub ja: Option<String>,
    pub ko: Option<String>,
    pub zh_hant: Option<String>,
    pub fr: Option<String>,
    pub de: Option<String>,
    pub es: Option<String>,
    pub it: Option<String>,
    pub ja_jp: Option<String>,
    pub zh_hans: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonLevelSkill {
    pub level: i64,
    pub skill: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonSkill {
    pub skill: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonDetail {
    pub level: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Promotion {
    pub result: String,
    pub details: Vec<PromotionDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvoItemMap {
    pub evo_sun_ribbon: u32,
    pub evo_lunar_ribbon: u32,
}

#[derive(Debug, Serialize, Deserialize)]

pub enum PromotionDetail {
    Level {
        level: u32,
    },
    SetForm {
        conditions: Vec<PromotionDetail>,
        form: u32,
    },
    Item {
        item_num: String,
    },
    Friendship {
        allies: u32,
    },
    MoveElement {
        move_element: String,
    },
    Move {
        move_num: String,
    },
    StatBoost {
        stat_boost_status: String,
    },
    Form {
        req_form: u32,
    },
    FormDusk {
        item_map: EvoItemMap,
    },
    Walk,
    MoveUse {
        last_move_status_id: String,
        move_repeat_status_id: String,
        move_num: String,
        amount: u32,
    },
    Gender {
        req_gender: u32,
    },
    Weather {
        weather: MapStatus,
    },
    Location {
        tile_element: String,
    },
    Personality {
        r#mod: u32,
        divisor: u32,
    },
    FormCream,
    LocOrigin,
    Hunger {
        hungry: bool,
    },
    KillCount {
        amount: u32,
    },
    Rescue,
    PartnerElement {
        partner_element: Element,
    },
    Crits {
        crit_status: String,
        stack: u32,
    },
    Money {
        amount: u32,
    },
    Partner {
        species: String,
    },
    FormScroll,
    TookDamage {
        amount: u32,
    },
    Shed {
        shed_species: String,
    },
    Stats {
        atk_def_comparison: i32,
    },
}
