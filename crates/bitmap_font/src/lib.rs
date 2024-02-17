pub mod loader;

use bevy::{asset::Asset, reflect::TypePath};
pub use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    usize,
};

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, PartialEq)]
pub struct BitmapFont {
    pub name: String,
    pub glyph_count: usize,
    pub size: (usize, usize),
    pub char_space: u8,
    pub char_height: u8,
    pub line_space: u8,
    pub glyphs: HashMap<u16, Glyph>,
    pub texture: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Glyph {
    pub code_point: u16,
    pub bounds: BoundingBox,
    pub colorless: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BoundingBox {
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

impl BitmapFont {
    /// Save the font sheet to a file
    pub fn save(&self, file: &mut File) -> Result<(), io::Error> {
        let buffer = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn load(buffer: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (font_sheet, _): (BitmapFont, usize) =
            bincode::serde::decode_from_slice(buffer, bincode::config::standard()).unwrap();
        Ok(font_sheet)
    }
}
