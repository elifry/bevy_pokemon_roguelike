pub mod loader;

use bevy::{
    asset::Asset,
    math::{Rect, Vec2},
    reflect::TypePath,
};
pub use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    usize,
};

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
}

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, PartialEq)]
pub struct FontSheetData {
    pub width: usize,
    pub height: usize,
    pub characters: HashMap<u32, GlyphData>,
}

impl FontSheetData {
    /// Save the font sheet to a file
    pub fn save(&self, file: &mut File) -> Result<(), io::Error> {
        let buffer = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn load(buffer: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (font_sheet, _): (FontSheetData, usize) =
            bincode::serde::decode_from_slice(buffer, bincode::config::standard()).unwrap();
        Ok(font_sheet)
    }
}
