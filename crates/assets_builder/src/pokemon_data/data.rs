use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonData {
    pub version: String,
    pub object: PokemonObject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonObject {
    #[serde(rename = "$type")]
    pub object_type: String,
    pub name: Name,
    pub released: bool,
    pub comment: String,
    pub title: Name,
    pub index_num: i64,
    #[serde(rename = "EXPTable")]
    pub exp_table: String,
    pub skill_group1: String,
    pub skill_group2: String,
    pub join_rate: i64,
    pub promote_from: String,
    pub promotions: Vec<Promotion>,
    pub forms: Vec<PokemonForm>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonForm {
    #[serde(rename = "$type")]
    pub form_type: String,
    pub released: bool,
    pub generation: i64,
    pub genderless_weight: i64,
    pub male_weight: i64,
    pub female_weight: i64,
    #[serde(rename = "BaseHP")]
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
    pub teach_skills: Vec<Skill>,
    pub shared_skills: Vec<Skill>,
    pub secret_skills: Vec<Skill>,
    pub form_name: Name,
    pub temporary: bool,
    pub promote_form: i64,
    pub element1: String,
    pub element2: String,
    pub intrinsic1: String,
    pub intrinsic2: String,
    pub intrinsic3: String,
    pub level_skills: Vec<LevelSkill>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    pub default_text: String,
    pub local_texts: LocalTexts,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LocalTexts {
    pub ja: String,
    pub ko: String,
    pub zh_hant: String,
    pub fr: String,
    pub de: String,
    pub es: String,
    pub it: String,
    pub ja_jp: String,
    pub zh_hans: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelSkill {
    pub level: i64,
    pub skill: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Skill {
    pub skill: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Promotion {
    pub result: String,
    pub details: Vec<Detail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Detail {
    #[serde(rename = "$type")]
    pub detail_type: String,
    pub level: i64,
}
