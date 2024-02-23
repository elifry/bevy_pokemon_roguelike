use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use anim_key::AnimKey;
use bevy::math::IVec2;
use orientation::Orientation;
use serde::{Deserialize, Serialize};

pub mod anim_key;
pub mod orientation;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct IVec2Serialized {
    x: i32,
    y: i32,
}

impl From<IVec2Serialized> for IVec2 {
    fn from(def: IVec2Serialized) -> Self {
        IVec2 { x: def.x, y: def.y }
    }
}

impl From<IVec2> for IVec2Serialized {
    fn from(vec: IVec2) -> Self {
        IVec2Serialized { x: vec.x, y: vec.y }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CharAnimationEntry {
    pub texture: Vec<u8>,
    pub index: usize,
    pub frame_width: u32,
    pub frame_height: u32,
    pub durations: Vec<u32>,
    pub rush_frame: Option<usize>,
    pub hit_frame: Option<usize>,
    pub return_frame: Option<usize>,
    // Offsets
    pub shadow_offsets: HashMap<Orientation, Vec<IVec2Serialized>>,
    pub body_offsets: HashMap<Orientation, Vec<IVec2Serialized>>,
    pub head_offsets: HashMap<Orientation, Vec<IVec2Serialized>>,
    pub left_offsets: HashMap<Orientation, Vec<IVec2Serialized>>,
    pub right_offsets: HashMap<Orientation, Vec<IVec2Serialized>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CharAnimation {
    pub anim: HashMap<AnimKey, CharAnimationEntry>,
}

impl CharAnimation {
    pub fn save(&self, file: &mut File) -> Result<(), io::Error> {
        let buffer = bincode::serde::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&buffer)?;
        Ok(())
    }

    pub fn load(buffer: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (font_sheet, _): (CharAnimation, usize) =
            bincode::serde::decode_from_slice(buffer, bincode::config::standard()).unwrap();
        Ok(font_sheet)
    }
}
