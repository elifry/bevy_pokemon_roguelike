use common::{element::Element, map_status::MapStatus};
use pokemon_data::{EvoItemMap, PokemonData, Promotion};
use serde::{Deserialize, Serialize};

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
