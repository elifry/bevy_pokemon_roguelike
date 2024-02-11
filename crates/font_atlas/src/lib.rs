use std::{collections::HashMap, fs::File, io::Write, usize};

use bevy_math::{Rect, Vec2};
use bincode::error::EncodeError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(remote = "Vec2")]
pub struct Vec2Ref {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(remote = "Rect")]
pub struct RectRef {
    #[serde(with = "Vec2Ref")]
    /// The minimum corner point of the rect.
    pub min: Vec2,
    #[serde(with = "Vec2Ref")]
    /// The maximum corner point of the rect.
    pub max: Vec2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GlyphData {
    #[serde(with = "RectRef")]
    pub rect: Rect,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FontSheet {
    pub width: usize,
    pub height: usize,
    pub characters: HashMap<u32, GlyphData>,
}

impl FontSheet {
    pub fn save(&self, file: &mut File) {
        let buffer = bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap();
        file.write_all(&buffer).unwrap();
    }

    pub fn load(buffer: &[u8]) -> Self {
        let result: (FontSheet, usize) =
            bincode::serde::decode_from_slice(buffer, bincode::config::standard()).unwrap();
        result.0
    }
}
