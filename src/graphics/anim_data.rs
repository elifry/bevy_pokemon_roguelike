use std::error::Error;

use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    math::Vec2,
    prelude::*,
    reflect::TypePath,
    utils::{hashbrown::HashMap, BoxedFuture},
};
use quick_xml::de::from_reader;
use serde::Deserialize;
use strum::{Display, IntoEnumIterator, IntoStaticStr};
use thiserror::Error;

use crate::pieces::Orientation;

pub struct AnimDataPlugin;

impl Plugin for AnimDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimData>()
            .init_asset_loader::<AnimDataLoader>();
    }
}

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
                let reference = self.anims.anim.get(&copy_of.copy_of).unwrap();
                match reference {
                    Anim::Value(reference_value) => reference_value,
                    Anim::CopyOf(_) => {
                        panic!("Can't copy {:?} for {:?}", copy_of.name, copy_of.copy_of)
                    }
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

#[derive(Debug, IntoStaticStr, Default, Display, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum AnimKey {
    Walk,
    Attack,
    Kick,
    Shoot,
    Strike,
    Sleep,
    Hurt,
    #[default]
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
    copy_of: Option<AnimKey>,
    // rush_frame: Option<i64>,
    // hit_frame: Option<i64>,
    // return_frame: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AnimValue {
    pub name: AnimKey,
    pub index: i64,
    pub frame_width: i64,
    pub frame_height: i64,
    pub durations: Durations,
    // rush_frame: Option<i64>,
    // hit_frame: Option<i64>,
    // return_frame: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimCopyOf {
    name: AnimKey,
    index: i64,
    copy_of: AnimKey,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Durations {
    pub duration: Vec<DurationValue>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DurationValue {
    #[serde(rename = "$text")]
    pub value: i64,
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
