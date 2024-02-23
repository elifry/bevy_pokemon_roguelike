use std::fmt;

use quick_xml::{de::from_reader, DeError};
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FontData {
    pub space_width: u32,
    pub char_height: u32,
    pub char_space: u32,
    pub line_space: u32,
    pub colorless: ColorlessGlyph,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColorlessGlyph {
    #[serde(rename = "Glyph", default)]
    #[serde(deserialize_with = "deserialize_hex_vec")]
    pub glyphs: Vec<u32>,
}

fn deserialize_hex_vec<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct HexVecVisitor;

    impl<'de> Visitor<'de> for HexVecVisitor {
        type Value = Vec<u32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of hex strings")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut vec = Vec::new();

            while let Some(hex_str) = seq.next_element::<String>()? {
                let num =
                    u32::from_str_radix(&hex_str[1..], 16).map_err(serde::de::Error::custom)?;
                vec.push(num);
            }

            Ok(vec)
        }
    }

    deserializer.deserialize_seq(HexVecVisitor)
}

impl FontData {
    pub fn parse_from_xml(font_data_content: &[u8]) -> Result<FontData, DeError> {
        let font_data: FontData = from_reader(font_data_content)?;
        Ok(font_data)
    }
}
