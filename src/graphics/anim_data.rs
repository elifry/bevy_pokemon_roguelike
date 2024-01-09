use std::error::{self, Error};

use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    math::Vec2,
    reflect::TypePath,
    utils::{hashbrown::HashMap, BoxedFuture},
};
use quick_xml::de::from_reader;
use serde::Deserialize;
use strum::IntoEnumIterator;
use thiserror::Error;

use super::Orientation;

#[derive(Asset, TypePath, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimData {
    pub shadow_size: i64,
    pub anims: Anims,
}

impl AnimData {
    pub fn get(&self, key: AnimKey) -> AnimInfo {
        AnimInfo {
            key,
            anims: &self.anims,
        }
    }
}

pub struct AnimInfo<'a> {
    key: AnimKey,
    anims: &'a Anims,
}

impl AnimInfo<'_> {
    pub fn columns(&self) -> usize {
        self.value().durations.duration.len()
    }

    pub fn rows(&self) -> usize {
        Orientation::iter().len()
    }

    pub fn tile_size(&self) -> Vec2 {
        Vec2::new(
            self.value().frame_width as f32,
            self.value().frame_height as f32,
        )
    }

    pub fn value(&self) -> &AnimValue {
        let anim = self.anims.anim.get(&self.key).unwrap();

        let value: &AnimValue = match anim {
            Anim::Value(value) => value,
            Anim::CopyOf(copy_of) => {
                let reference = self.anims.anim.get(&copy_of.name).unwrap();
                match reference {
                    Anim::Value(reference_value) => reference_value,
                    Anim::CopyOf(_) => panic!("Can't copy of copy of"),
                }
            }
        };

        value
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "AnimsRaw")]
pub struct Anims {
    anim: HashMap<AnimKey, Anim>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimsRaw {
    pub anim: Vec<Anim>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum AnimKey {
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
    TailWhip,
    Faint,
    HitGround,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "AnimRaw")]
pub enum Anim {
    Value(AnimValue),
    CopyOf(AnimCopyOf),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AnimRaw {
    name: AnimKey,
    index: i64,
    frame_width: Option<i64>,
    frame_height: Option<i64>,
    durations: Option<Durations>,
    copy_of: Option<String>,
    rush_frame: Option<i64>,
    hit_frame: Option<i64>,
    return_frame: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimValue {
    name: AnimKey,
    index: i64,
    frame_width: i64,
    frame_height: i64,
    durations: Durations,
    // rush_frame: Option<i64>,
    // hit_frame: Option<i64>,
    // return_frame: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimCopyOf {
    name: AnimKey,
    index: i64,
    copy_of: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Durations {
    pub duration: Vec<Duration>,
}

#[derive(Debug, Deserialize)]
pub struct Duration {
    #[serde(rename = "$text")]
    pub value: String,
}

// Dirty hack for enum deserialize issue: https://github.com/tafia/quick-xml/issues/203
impl std::convert::TryFrom<AnimRaw> for Anim {
    type Error = &'static str;
    fn try_from(anim_raw: AnimRaw) -> Result<Self, Self::Error> {
        if anim_raw.copy_of.is_some() && anim_raw.frame_width.is_some() {
            return Err("AnimRaw cannot be both CopyOf and Value.");
        }
        if let Some(copy_of) = anim_raw.copy_of {
            return Ok(Self::CopyOf(AnimCopyOf {
                name: anim_raw.name,
                index: anim_raw.index,
                copy_of,
            }));
        }
        if let (Some(frame_width), Some(frame_height), Some(durations)) = (
            anim_raw.frame_width,
            anim_raw.frame_height,
            anim_raw.durations,
        ) {
            return Ok(Self::Value(AnimValue {
                name: anim_raw.name,
                index: anim_raw.index,
                durations,
                frame_width,
                frame_height,
            }));
        }
        Err("Anim is not AnimValue or AnimCopyOf.")
    }
}

impl std::convert::TryFrom<AnimsRaw> for Anims {
    type Error = &'static str;
    fn try_from(anims_raw: AnimsRaw) -> Result<Self, Self::Error> {
        let anim_map = anims_raw
            .anim
            .into_iter()
            .map(|anim| {
                let anim_key = match &anim {
                    Anim::Value(value) => value.name,
                    Anim::CopyOf(value) => value.name,
                };

                (anim_key, anim)
            })
            .collect::<HashMap<_, _>>();

        Ok(Anims { anim: anim_map })
    }
}

impl AnimData {
    fn parse_from_xml(anim_data_content: &[u8]) -> Result<AnimData, Box<dyn Error>> {
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
        &["anim.xml"]
    }
}
