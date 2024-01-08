use std::error::{self, Error};

use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};
use quick_xml::de::from_reader;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimData {
    shadow_size: i64,
    anims: Anims,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Anims {
    anim: Vec<Anim>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnimName {
    Walk,
    Attack,
    Kick,
    Shoot,
    Strike,
    Sleep,
    Hurt,
    Idle,
    Swing,
    Double,
    Hop,
    Charge,
    Rotate,
    EventSleep,
    Wake,
    Eat,
    Tumble,
    Pose,
    Pull,
    Pain,
    Float,
    DeepBreath,
    Nod,
    Sit,
    LookUp,
    Sink,
    Trip,
    Laying,
    LeapForth,
    Head,
    Cringe,
    LostBalance,
    TumbleBack,
    Faint,
    HitGround,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "AnimRaw")]
pub enum Anim {
    Value {
        name: String,
        index: i64,
        frame_width: i64,
        frame_height: i64,
        durations: Durations,
        // rush_frame: Option<i64>,
        // hit_frame: Option<i64>,
        // return_frame: Option<i64>,
    },
    CopyOf {
        name: String,
        index: i64,
        copy_of: String,
    },
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AnimRaw {
    name: String,
    index: i64,
    frame_width: Option<i64>,
    frame_height: Option<i64>,
    durations: Option<Durations>,
    copy_of: Option<String>,
    rush_frame: Option<i64>,
    hit_frame: Option<i64>,
    return_frame: Option<i64>,
}

// Dirty hack for enum deserialize issue: https://github.com/tafia/quick-xml/issues/203
impl std::convert::TryFrom<AnimRaw> for Anim {
    type Error = &'static str;
    fn try_from(anim_raw: AnimRaw) -> Result<Self, Self::Error> {
        if anim_raw.copy_of.is_some() && anim_raw.frame_width.is_some() {
            return Err("AnimRaw cannot be both CopyOf and Value.");
        }
        if let Some(copy_of) = anim_raw.copy_of {
            return Ok(Self::CopyOf {
                name: anim_raw.name,
                index: anim_raw.index,
                copy_of,
            });
        }
        if let (Some(frame_width), Some(frame_height), Some(durations)) = (
            anim_raw.frame_width,
            anim_raw.frame_height,
            anim_raw.durations,
        ) {
            return Ok(Self::Value {
                name: anim_raw.name,
                index: anim_raw.index,
                durations,
                frame_width,
                frame_height,
            });
        }
        Err("Anim is not AnimValue or AnimCopyOf.")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimValue {
    name: String,
    index: i64,
    frame_width: i64,
    frame_height: i64,
    durations: Durations,
    rush_frame: Option<i64>,
    hit_frame: Option<i64>,
    return_frame: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimCopyOf {
    name: String,
    index: i64,
    copy_of: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Durations {
    duration: Vec<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Duration {
    #[serde(rename = "$text")]
    value: String,
}

impl AnimData {
    pub fn parse_from_xml(anim_data_content: &[u8]) -> Result<AnimData, Box<dyn Error>> {
        let anim_data: AnimData = from_reader(anim_data_content)?;

        Ok(anim_data)
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AnimDataLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    // #[error("Could not parse the asset {0}")]
    // XmlParseError(#[from] std::error::Error),
}

#[derive(Default)]
pub struct AnimDataLoader;

impl AssetLoader for AnimDataLoader {
    type Asset = AnimData;
    type Settings = ();
    type Error = AnimDataLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let anim_data = AnimData::parse_from_xml(&bytes).unwrap();

            Ok(anim_data)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim-data.xml"]
    }
}
