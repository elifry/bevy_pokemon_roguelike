use std::{
    fs::File,
    io::{BufWriter, Write},
};

use bevy_math::{IVec2, UVec2};
use serde::{Deserialize, Serialize};

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
pub struct CharAnimation {
    pub texture: Vec<u8>,
    pub index: usize,
    pub frame_width: u32,
    pub frame_height: u32,
    pub durations: Vec<u32>,
    pub rush_frame: Option<usize>,
    pub hit_frame: Option<usize>,
    pub return_frame: Option<usize>,
    // Offsets
    pub shadow_offsets: Vec<Vec<IVec2Serialized>>,
    pub body_offsets: Vec<Vec<IVec2Serialized>>,
    pub head_offsets: Vec<Vec<IVec2Serialized>>,
    pub left_offsets: Vec<Vec<IVec2Serialized>>,
    pub right_offsets: Vec<Vec<IVec2Serialized>>,
}

impl CharAnimation {
    /// Save the font sheet to a file
    pub fn save(&self, file: &mut File) -> Result<(), ()> {
        let xml = quick_xml::se::to_string(&self).expect("Failed to serialize to XML");
        file.write_all(xml.as_bytes())
            .expect("Failed to write XML to file");

        Ok(())
    }

    // pub fn load(buffer: &[u8]) -> Result<Self, bincode::error::DecodeError> {
    //     let (font_sheet, _): (Font, usize) =
    //         bincode::serde::decode_from_slice(buffer, bincode::config::standard()).unwrap();
    //     Ok(font_sheet)
    // }
}
