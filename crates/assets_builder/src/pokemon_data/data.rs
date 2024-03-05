use common::{element::Element, map_status::MapStatus};
use pokemon_data::{EvoItemMap, PokemonData, Promotion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawPokemonData {
    pub version: String,
    pub object: RawPokemonObject,
}

impl RawPokemonData {
    pub fn parse_from_json(pokemon_data: &[u8]) -> Result<RawPokemonData, serde_json::Error> {
        let font_data: RawPokemonData = serde_json::from_reader(pokemon_data)?;
        Ok(font_data)
    }
    pub fn to_data(&self) -> PokemonData {
        pokemon_data::PokemonData {
            name: self.object.name.to_data(),
            released: self.object.released,
            comment: self.object.comment.to_owned(),
            title: self.object.title.to_data(),
            index_num: self.object.index_num,
            exp_table: self.object.exp_table.to_owned(),
            skill_group1: self.object.skill_group1.to_owned(),
            skill_group2: self.object.skill_group2.to_owned(),
            join_rate: self.object.join_rate,
            promote_from: self.object.promote_from.to_owned(),
            promotions: self
                .object
                .promotions
                .iter()
                .map(|promotion| promotion.to_data())
                .collect(),
            forms: self
                .object
                .forms
                .iter()
                .map(|form| form.to_data())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawPokemonObject {
    #[serde(rename = "$type")]
    pub object_type: String,
    pub name: RawTextData,
    pub released: bool,
    pub comment: String,
    pub title: RawTextData,
    pub index_num: i64,
    #[serde(rename = "EXPTable")]
    pub exp_table: String,
    pub skill_group1: String,
    pub skill_group2: String,
    pub join_rate: i64,
    pub promote_from: String,
    pub promotions: Vec<RawPromotion>,
    pub forms: Vec<PokemonRawForm>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonRawForm {
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
    pub teach_skills: Vec<PokemonRawSkill>,
    pub shared_skills: Vec<PokemonRawSkill>,
    pub secret_skills: Vec<PokemonRawSkill>,
    pub form_name: RawTextData,
    pub temporary: bool,
    pub promote_form: i64,
    pub element1: String,
    pub element2: String,
    pub intrinsic1: String,
    pub intrinsic2: String,
    pub intrinsic3: String,
    pub level_skills: Vec<PokemonRawLevelSkill>,
}

impl PokemonRawForm {
    pub fn to_data(&self) -> pokemon_data::PokemonForm {
        pokemon_data::PokemonForm {
            released: self.released,
            generation: self.generation,
            genderless_weight: self.genderless_weight,
            male_weight: self.male_weight,
            female_weight: self.female_weight,
            base_hp: self.base_hp,
            base_atk: self.base_atk,
            base_def: self.base_def,
            base_m_atk: self.base_m_atk,
            base_m_def: self.base_m_def,
            base_speed: self.base_speed,
            exp_yield: self.exp_yield,
            height: self.height,
            weight: self.weight,
            personalities: self.personalities.clone(),
            teach_skills: self
                .teach_skills
                .iter()
                .map(|skill| skill.to_data())
                .collect(),
            shared_skills: self
                .shared_skills
                .iter()
                .map(|skill| skill.to_data())
                .collect(),
            secret_skills: self
                .secret_skills
                .iter()
                .map(|skill| skill.to_data())
                .collect(),
            form_name: self.form_name.to_data(),
            temporary: self.temporary,
            promote_form: self.promote_form,
            element1: self.element1.to_owned(),
            element2: self.element2.to_owned(),
            intrinsic1: self.intrinsic1.to_owned(),
            intrinsic2: self.intrinsic2.to_owned(),
            intrinsic3: self.intrinsic3.to_owned(),
            level_skills: self
                .level_skills
                .iter()
                .map(|skill| skill.to_data())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawTextData {
    pub default_text: String,
    pub local_texts: RawLocalTexts,
}

impl RawTextData {
    pub fn to_data(&self) -> pokemon_data::TextData {
        pokemon_data::TextData {
            default_text: self.default_text.to_owned(),
            local_texts: self.local_texts.to_data(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RawLocalTexts {
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

impl RawLocalTexts {
    pub fn to_data(&self) -> pokemon_data::LocalTexts {
        let format_str = |text: &Option<String>| {
            if text.as_ref().map_or(true, |t| t.is_empty()) {
                return None;
            }
            text.clone()
        };

        pokemon_data::LocalTexts {
            ja: format_str(&self.ja),
            ko: format_str(&self.ko),
            zh_hant: format_str(&self.zh_hant),
            fr: format_str(&self.fr),
            de: format_str(&self.de),
            es: format_str(&self.es),
            it: format_str(&self.it),
            ja_jp: format_str(&self.ja_jp),
            zh_hans: format_str(&self.zh_hans),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonRawLevelSkill {
    pub level: i64,
    pub skill: String,
}

impl PokemonRawLevelSkill {
    pub fn to_data(&self) -> pokemon_data::PokemonLevelSkill {
        pokemon_data::PokemonLevelSkill {
            level: self.level,
            skill: self.skill.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PokemonRawSkill {
    pub skill: String,
}

impl PokemonRawSkill {
    pub fn to_data(&self) -> pokemon_data::PokemonSkill {
        pokemon_data::PokemonSkill {
            skill: self.skill.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawPokemonDetail {
    #[serde(rename = "$type")]
    pub detail_type: String,
    pub level: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawPromotion {
    pub result: String,
    pub details: Vec<RawPromotionDetail>,
}

impl RawPromotion {
    pub fn to_data(&self) -> pokemon_data::Promotion {
        pokemon_data::Promotion {
            result: self.result.to_owned(),
            details: self.details.iter().map(|detail| detail.to_data()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]

pub enum RawPromotionDetail {
    #[serde(rename = "PMDC.Data.EvoLevel, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Level { level: u32 },
    #[serde(rename = "PMDC.Data.EvoSetForm, PMDC")]
    #[serde(rename_all = "PascalCase")]
    SetForm {
        conditions: Vec<RawPromotionDetail>,
        form: u32,
    },
    #[serde(rename = "PMDC.Data.EvoItem, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Item { item_num: String },
    #[serde(rename = "PMDC.Data.EvoFriendship, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Friendship { allies: u32 },
    #[serde(rename = "PMDC.Data.EvoMoveElement, PMDC")]
    #[serde(rename_all = "PascalCase")]
    MoveElement { move_element: String },
    #[serde(rename = "PMDC.Data.EvoMove, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Move { move_num: String },
    #[serde(rename = "PMDC.Data.EvoStatBoost, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StatBoost { stat_boost_status: String },
    #[serde(rename = "PMDC.Data.EvoForm, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Form { req_form: u32 },
    #[serde(rename = "PMDC.Data.EvoFormDusk, PMDC")]
    #[serde(rename_all = "PascalCase")]
    FormDusk { item_map: EvoItemMap },
    #[serde(rename = "PMDC.Data.EvoWalk, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Walk,
    #[serde(rename = "PMDC.Data.EvoMoveUse, PMDC")]
    #[serde(rename_all = "PascalCase")]
    MoveUse {
        #[serde(rename = "LastMoveStatusID")]
        last_move_status_id: String,
        #[serde(rename = "MoveRepeatStatusID")]
        move_repeat_status_id: String,
        move_num: String,
        amount: u32,
    },
    #[serde(rename = "PMDC.Data.EvoGender, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Gender { req_gender: u32 },
    #[serde(rename = "PMDC.Data.EvoWeather, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Weather { weather: MapStatus },
    #[serde(rename = "PMDC.Data.EvoLocation, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Location { tile_element: String },
    #[serde(rename = "PMDC.Data.EvoPersonality, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Personality {
        #[serde(rename = "Mod")]
        r#mod: u32,
        divisor: u32,
    },
    #[serde(rename = "PMDC.Data.EvoFormCream, PMDC")]
    #[serde(rename_all = "PascalCase")]
    FormCream,
    #[serde(rename = "PMDC.Data.EvoFormLocOrigin, PMDC")]
    #[serde(rename_all = "PascalCase")]
    LocOrigin,
    #[serde(rename = "PMDC.Data.EvoHunger, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Hunger { hungry: bool },
    #[serde(rename = "PMDC.Data.EvoKillCount, PMDC")]
    #[serde(rename_all = "PascalCase")]
    KillCount { amount: u32 },
    #[serde(rename = "PMDC.Data.EvoRescue, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Rescue,
    #[serde(rename = "PMDC.Data.EvoPartnerElement, PMDC")]
    #[serde(rename_all = "PascalCase")]
    PartnerElement { partner_element: Element },
    #[serde(rename = "PMDC.Data.EvoCrits, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Crits { crit_status: String, stack: u32 },
    #[serde(rename = "PMDC.Data.EvoMoney, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Money { amount: u32 },
    #[serde(rename = "PMDC.Data.EvoPartner, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Partner { species: String },
    #[serde(rename = "PMDC.Data.EvoFormScroll, PMDC")]
    #[serde(rename_all = "PascalCase")]
    FormScroll,
    #[serde(rename = "PMDC.Data.EvoTookDamage, PMDC")]
    #[serde(rename_all = "PascalCase")]
    TookDamage { amount: u32 },
    #[serde(rename = "PMDC.Data.EvoShed, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Shed { shed_species: String },
    #[serde(rename = "PMDC.Data.EvoStats, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Stats { atk_def_comparison: i32 },
}

impl RawPromotionDetail {
    pub fn to_data(&self) -> pokemon_data::PromotionDetail {
        match self {
            RawPromotionDetail::Level { level } => {
                pokemon_data::PromotionDetail::Level { level: *level }
            }
            RawPromotionDetail::SetForm { conditions, form } => {
                pokemon_data::PromotionDetail::SetForm {
                    form: *form,
                    conditions: conditions
                        .iter()
                        .map(|condition| condition.to_data())
                        .collect(),
                }
            }
            RawPromotionDetail::Item { item_num } => pokemon_data::PromotionDetail::Item {
                item_num: item_num.to_owned(),
            },
            RawPromotionDetail::Friendship { allies } => {
                pokemon_data::PromotionDetail::Friendship { allies: *allies }
            }
            RawPromotionDetail::MoveElement { move_element } => {
                pokemon_data::PromotionDetail::MoveElement {
                    move_element: move_element.to_owned(),
                }
            }
            RawPromotionDetail::Move { move_num } => pokemon_data::PromotionDetail::Move {
                move_num: move_num.to_owned(),
            },
            RawPromotionDetail::StatBoost { stat_boost_status } => {
                pokemon_data::PromotionDetail::StatBoost {
                    stat_boost_status: stat_boost_status.to_owned(),
                }
            }
            RawPromotionDetail::Form { req_form } => pokemon_data::PromotionDetail::Form {
                req_form: *req_form,
            },
            RawPromotionDetail::FormDusk { item_map } => pokemon_data::PromotionDetail::FormDusk {
                item_map: item_map.clone(),
            },
            RawPromotionDetail::Walk => pokemon_data::PromotionDetail::Walk,
            RawPromotionDetail::MoveUse {
                last_move_status_id,
                move_repeat_status_id,
                move_num,
                amount,
            } => pokemon_data::PromotionDetail::MoveUse {
                last_move_status_id: last_move_status_id.to_owned(),
                move_repeat_status_id: move_repeat_status_id.to_owned(),
                move_num: move_num.to_owned(),
                amount: *amount,
            },
            RawPromotionDetail::Gender { req_gender } => pokemon_data::PromotionDetail::Gender {
                req_gender: *req_gender,
            },
            RawPromotionDetail::Weather { weather } => {
                pokemon_data::PromotionDetail::Weather { weather: *weather }
            }
            RawPromotionDetail::Location { tile_element } => {
                pokemon_data::PromotionDetail::Location {
                    tile_element: tile_element.to_owned(),
                }
            }
            RawPromotionDetail::Personality { r#mod, divisor } => {
                pokemon_data::PromotionDetail::Personality {
                    r#mod: *r#mod,
                    divisor: *divisor,
                }
            }
            RawPromotionDetail::FormCream => pokemon_data::PromotionDetail::FormCream,
            RawPromotionDetail::LocOrigin => pokemon_data::PromotionDetail::LocOrigin,
            RawPromotionDetail::Hunger { hungry } => {
                pokemon_data::PromotionDetail::Hunger { hungry: *hungry }
            }
            RawPromotionDetail::KillCount { amount } => {
                pokemon_data::PromotionDetail::KillCount { amount: *amount }
            }
            RawPromotionDetail::Rescue => pokemon_data::PromotionDetail::Rescue,
            RawPromotionDetail::PartnerElement { partner_element } => {
                pokemon_data::PromotionDetail::PartnerElement {
                    partner_element: *partner_element,
                }
            }
            RawPromotionDetail::Crits { crit_status, stack } => {
                pokemon_data::PromotionDetail::Crits {
                    crit_status: crit_status.to_owned(),
                    stack: *stack,
                }
            }
            RawPromotionDetail::Money { amount } => {
                pokemon_data::PromotionDetail::Money { amount: *amount }
            }
            RawPromotionDetail::Partner { species } => pokemon_data::PromotionDetail::Partner {
                species: species.to_owned(),
            },
            RawPromotionDetail::FormScroll => pokemon_data::PromotionDetail::FormScroll,
            RawPromotionDetail::TookDamage { amount } => {
                pokemon_data::PromotionDetail::TookDamage { amount: *amount }
            }
            RawPromotionDetail::Shed { shed_species } => pokemon_data::PromotionDetail::Shed {
                shed_species: shed_species.to_owned(),
            },
            RawPromotionDetail::Stats { atk_def_comparison } => {
                pokemon_data::PromotionDetail::Stats {
                    atk_def_comparison: *atk_def_comparison,
                }
            }
        }
    }
}
