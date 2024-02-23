use std::collections::HashMap;

use bevy_math::{UVec2, Vec2};
use quick_xml::{de::from_reader, DeError};
use serde::Deserialize;
use strum::{Display, IntoEnumIterator, IntoStaticStr};

use super::orientation::Orientation;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimData {
    pub shadow_size: i32,
    pub anims: Anims,
}

impl AnimData {
    pub fn get<'a>(&'a self, key: &'a AnimKey) -> AnimInfo<'a> {
        AnimInfo {
            key,
            anims: &self.anims,
        }
    }
}

pub struct AnimInfo<'a> {
    pub key: &'a AnimKey,
    anims: &'a Anims,
}

impl AnimInfo<'_> {
    pub fn columns(&self) -> usize {
        self.value().durations.duration.len()
    }

    pub fn rows(&self) -> usize {
        match self.value().name {
            // No orientations for this ones
            AnimKey::Sleep => 1,
            AnimKey::TumbleBack => 1,
            AnimKey::Sink => 1,
            AnimKey::Pull => 1,
            AnimKey::LostBalance => 1,
            AnimKey::DeepBreath => 1,
            AnimKey::Sit => 1,
            AnimKey::HitGround => 1,
            AnimKey::Tumble => 1,
            AnimKey::LookUp => 1,
            AnimKey::LeapForth => 1,
            AnimKey::Cringe => 1,
            AnimKey::Eat => 1,
            _ => Orientation::iter().len(),
        }
    }

    pub fn tile_size(&self) -> UVec2 {
        UVec2::new(self.value().frame_width, self.value().frame_height)
    }

    pub fn index(&self) -> usize {
        let anim = self.anims.anim.get(self.key).unwrap();

        let index = match anim {
            Anim::Value(value) => value.index,
            Anim::CopyOf(copy_of) => copy_of.index.unwrap_or(self.value().index),
        };
        index
    }

    pub fn value(&self) -> &AnimValue {
        let anim = self.anims.anim.get(self.key).unwrap();

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
    pub anim: HashMap<AnimKey, Anim>,
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
    Dance,
    Shake,
    SpAttack,
    Twirl,
    Withdraw,
    Ricochet,
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
    index: Option<usize>,
    frame_width: Option<u32>,
    frame_height: Option<u32>,
    durations: Option<Durations>,
    copy_of: Option<AnimKey>,
    rush_frame: Option<usize>,
    hit_frame: Option<usize>,
    return_frame: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AnimValue {
    pub name: AnimKey,
    pub index: usize,
    pub frame_width: u32,
    pub frame_height: u32,
    pub durations: Durations,
    pub rush_frame: Option<usize>,
    pub hit_frame: Option<usize>,
    pub return_frame: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimCopyOf {
    name: AnimKey,
    index: Option<usize>,
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
    pub value: u32,
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
        if let (Some(frame_width), Some(frame_height), Some(durations), Some(index)) = (
            anim_raw.frame_width,
            anim_raw.frame_height,
            anim_raw.durations,
            anim_raw.index,
        ) {
            return Ok(Self::Value(AnimValue {
                name: anim_raw.name,
                index,
                durations,
                frame_width,
                frame_height,
                rush_frame: anim_raw.rush_frame,
                hit_frame: anim_raw.hit_frame,
                return_frame: anim_raw.return_frame,
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
    pub fn parse_from_xml(anim_data_content: &[u8]) -> Result<AnimData, DeError> {
        let anim_data: AnimData = from_reader(anim_data_content)?;
        Ok(anim_data)
    }
}
