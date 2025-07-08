use std::collections::HashMap;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext, LoadedFolder},
    prelude::*,
    utils::BoxedFuture,
};
use char_animation::anim_key::AnimKey;
use serde::Deserialize;
use serde_json;
use thiserror::Error;

use crate::{
    loading::AssetsLoading,
    spells::{MoveCategory, ProjectileSpell, Spell, SpellCast, SpellHit, SpellType},
    utils::get_path_from_handle,
    GameState,
};

const SPELL_DATA_PATH: &str = "data/spells";

pub struct SpellDataPlugin;

impl Plugin for SpellDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpellData>()
            .init_asset_loader::<SpellDataLoader>()
            .init_resource::<SpellDataAssetsFolder>()
            .init_resource::<SpellDataLookup>()
            .add_systems(OnEnter(GameState::Loading), load_spell_assets_folder)
            .add_systems(OnEnter(GameState::AssetsLoaded), process_spell_data_assets);
    }
}

#[derive(Default, Resource)]
struct SpellDataAssetsFolder(Handle<LoadedFolder>);

/// Lookup for spell name -> Spell data
#[derive(Resource, Debug, Default)]
pub struct SpellDataLookup(pub HashMap<String, Spell>);

// JSON data structures matching the Pokemon spell format
#[derive(Asset, Debug, TypePath, Deserialize)]
pub struct SpellData {
    #[serde(rename = "Object")]
    pub object: SpellObject,
}

#[derive(Debug, Deserialize)]
pub struct SpellObject {
    #[serde(rename = "Name")]
    pub name: SpellName,
    #[serde(rename = "HitboxAction")]
    pub hitbox_action: HitboxAction,
    #[serde(rename = "Data")]
    pub data: SpellDataInfo,
}

#[derive(Debug, Deserialize)]
pub struct SpellName {
    #[serde(rename = "DefaultText")]
    pub default_text: String,
}

#[derive(Debug, Deserialize)]
pub struct HitboxAction {
    #[serde(rename = "$type")]
    pub action_type: String,
    #[serde(rename = "Range", default)]
    pub range: Option<i32>,
    #[serde(rename = "StreamEmitter", default)]
    pub stream_emitter: Option<StreamEmitter>,
    #[serde(rename = "ActionFX", default)]
    pub action_fx: Option<ActionFX>,
    #[serde(rename = "TileEmitter", default)]
    pub tile_emitter: Option<TileEmitter>,
    #[serde(rename = "HitTiles", default)]
    pub hit_tiles: Option<bool>,
    #[serde(rename = "BurstTiles", default)]
    pub burst_tiles: Option<i32>,
    #[serde(rename = "Emitter", default)]
    pub emitter: Option<Emitter>,
    #[serde(rename = "WideAngle", default)]
    pub wide_angle: Option<i32>,
    #[serde(rename = "TargetAlignments", default)]
    pub target_alignments: Option<i32>,
    #[serde(rename = "LagBehindTime", default)]
    pub lag_behind_time: Option<i32>,
    #[serde(rename = "PreActions", default)]
    pub pre_actions: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct StreamEmitter {
    #[serde(rename = "Anims", default)]
    pub anims: Vec<AnimInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AnimInfo {
    #[serde(rename = "Anim", default)]
    pub anim: Option<AnimData>,
    // Also support direct animation fields for cases where there's no "Anim" wrapper
    #[serde(rename = "AnimIndex", default)]
    pub anim_index: Option<String>,
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>, // Catch all other animation fields
}

#[derive(Debug, Deserialize)]
pub struct AnimData {
    #[serde(rename = "AnimIndex")]
    pub anim_index: String,
}

#[derive(Debug, Deserialize)]
pub struct ActionFX {
    #[serde(rename = "Emitter")]
    pub emitter: Option<Emitter>,
}

#[derive(Debug, Deserialize)]
pub struct Emitter {
    #[serde(rename = "Anim", default)]
    pub anim: Option<serde_json::Value>, // Accept any JSON structure for Anim
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>, // Catch all other fields
}

#[derive(Debug, Deserialize)]
pub struct AnimContainer {
    #[serde(rename = "Anim")]
    pub anim: AnimData,
}

#[derive(Debug, Deserialize)]
pub struct TileEmitter {
    #[serde(rename = "Anim", default)]
    pub anim: Option<serde_json::Value>, // Accept any JSON structure for Anim
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>, // Catch all other fields
}

#[derive(Debug, Deserialize)]
pub struct SpellDataInfo {
    #[serde(rename = "Element")]
    pub element: String,
    #[serde(rename = "Category")]
    pub category: i32,
    #[serde(rename = "SkillStates", default)]
    pub skill_states: Vec<SkillState>,
    #[serde(rename = "HitFX", default)]
    pub hit_fx: Option<HitFX>,
}

#[derive(Debug, Deserialize)]
pub struct SkillState {
    #[serde(rename = "$type")]
    pub state_type: String,
    #[serde(rename = "Power")]
    pub power: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct HitFX {
    #[serde(rename = "Emitter", default)]
    pub emitter: Option<HitEmitter>,
}

#[derive(Debug, Deserialize)]
pub struct HitEmitter {
    #[serde(rename = "Anim", default)]
    pub anim: Option<serde_json::Value>, // Accept any JSON structure for Anim
    #[serde(rename = "Anims", default)]
    pub anims: Option<Vec<AnimInfo>>,
    #[serde(flatten)]
    pub other_fields: HashMap<String, serde_json::Value>, // Catch all other fields
}

// Asset loader
#[derive(Default)]
pub struct SpellDataLoader;

#[derive(Debug, Error)]
pub enum SpellDataLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl AssetLoader for SpellDataLoader {
    type Asset = SpellData;
    type Settings = ();
    type Error = SpellDataLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            // Remove UTF-8 BOM if present (same logic as assets_builder)
            let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
                // Remove the first three bytes (the BOM)
                bytes[3..].to_vec()
            } else {
                bytes
            };

            let spell_data: SpellData = serde_json::from_slice(&bytes)?;
            Ok(spell_data)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

fn load_spell_assets_folder(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut spell_data_assets_folder: ResMut<SpellDataAssetsFolder>,
) {
    info!("spell data assets loading...");

    let spell_data_folder = asset_server.load_folder(SPELL_DATA_PATH);
    loading.0.push(spell_data_folder.clone().untyped());
    spell_data_assets_folder.0 = spell_data_folder;
}

fn process_spell_data_assets(
    spell_data_assets_folder: Res<SpellDataAssetsFolder>,
    mut spell_data_lookup: ResMut<SpellDataLookup>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    spell_data_assets: Res<Assets<SpellData>>,
    mut commands: Commands,
) {
    info!("Processing spell data assets...");

    let folder: &LoadedFolder = match loaded_folder_assets.get(&spell_data_assets_folder.0) {
        Some(folder) => folder,
        None => {
            error!("Couldn't load the spell data folder");
            return;
        }
    };

    info!("Found {} spell files to process", folder.handles.len());

    let mut successful_loads = Vec::new();
    let mut failed_loads = Vec::new();
    let mut parse_failures = Vec::new();

    for handle in &folder.handles {
        let Some(path) = get_path_from_handle(handle) else {
            continue;
        };

        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let Some(index) = file_name.find('.') else {
            continue;
        };

        // Get the spell name from filename (e.g., "flamethrower.json" -> "flamethrower")
        let spell_name = file_name[..index].to_string();

        // Try to get the typed handle
        let data_handle = match handle.clone().try_typed::<SpellData>() {
            Ok(handle) => handle,
            Err(_) => {
                failed_loads.push(spell_name.clone());
                warn!("Failed to get typed handle for {}", spell_name);
                continue;
            }
        };

        // Try to get the spell data asset
        let spell_data = match spell_data_assets.get(&data_handle) {
            Some(data) => data,
            None => {
                failed_loads.push(spell_name.clone());
                warn!(
                    "Failed to load spell data asset for {} (likely JSON parsing error)",
                    spell_name
                );
                continue;
            }
        };

        // Try to convert SpellData to our Spell struct
        match convert_spell_data_to_spell(&spell_name, spell_data) {
            Ok(spell) => {
                successful_loads.push(spell_name.clone());
                spell_data_lookup.0.insert(spell_name.clone(), spell);
                // info!("✓ Successfully loaded spell: {}", spell_name);
            }
            Err(e) => {
                warn!("✗ Failed to convert spell data for {}: {}", spell_name, e);
                parse_failures.push((spell_name.clone(), e));
            }
        }
    }

    // Provide detailed report
    info!("=== SPELL LOADING REPORT ===");
    info!("Successfully loaded: {} spells", successful_loads.len());
    info!(
        "Failed to load (JSON errors): {} spells",
        failed_loads.len()
    );
    info!(
        "Failed to parse (struct conversion): {} spells",
        parse_failures.len()
    );

    // if successful_loads.len() > 0 {
    //     info!("✓ Successfully loaded spells: {:?}", successful_loads);
    // }

    if failed_loads.len() > 0 {
        warn!("✗ Failed to load (JSON parsing errors): {:?}", failed_loads);
    }

    if parse_failures.len() > 0 {
        warn!("✗ Failed to parse (conversion errors):");
        for (name, error) in &parse_failures {
            warn!("  - {}: {}", name, error);
        }
    }

    info!(
        "Final spell lookup contains {} spells",
        spell_data_lookup.0.len()
    );

    commands.remove_resource::<SpellDataAssetsFolder>();
}

fn convert_spell_data_to_spell(spell_name: &str, data: &SpellData) -> Result<Spell, String> {
    let obj = &data.object;

    // Extract base power from skill states
    let base_power = obj
        .data
        .skill_states
        .iter()
        .find(|state| state.state_type.contains("BasePowerState"))
        .and_then(|state| state.power)
        .unwrap_or(0); // Default to 0 for status moves

    // Convert category (1 = Physical, 2 = Special, 3 = Status, etc.)
    let category = match obj.data.category {
        1 => MoveCategory::Physical,
        2 => MoveCategory::Special,
        _ => MoveCategory::Physical, // Default to Physical for now
    };

    // Get range (default to 1 if not specified)
    let range = obj.hitbox_action.range.unwrap_or(1);
    let range = 1..=range.max(1);

    // Extract visual effects
    let (projectile_effect, hit_effect, cast_effect) =
        extract_visual_effects(&obj.hitbox_action, &obj.data)?;

    Ok(Spell {
        name: leak_string(obj.name.default_text.clone()),
        move_type: leak_string(obj.data.element.clone()),
        base_power,
        category,
        range,
        spell_type: SpellType::Projectile(ProjectileSpell {
            visual_effect: projectile_effect,
        }),
        hit: SpellHit {
            visual_effect: hit_effect,
        },
        cast: SpellCast {
            visual_effect: cast_effect,
            animation: AnimKey::Shoot, // Default animation
        },
    })
}

fn extract_visual_effects(
    hitbox_action: &HitboxAction,
    data: &SpellDataInfo,
) -> Result<(&'static str, &'static str, &'static str), String> {
    // Try to get effect from various sources, in order of preference
    let projectile_effect = if let Some(stream_emitter) = &hitbox_action.stream_emitter {
        // ProjectileAction - get from StreamEmitter
        if let Some(first_anim) = stream_emitter.anims.first() {
            // Handle both wrapped and direct animation data
            if let Some(anim) = &first_anim.anim {
                leak_string(anim.anim_index.clone())
            } else if let Some(anim_index) = &first_anim.anim_index {
                leak_string(anim_index.clone())
            } else {
                get_default_projectile_effect(&data.element)
            }
        } else {
            get_default_projectile_effect(&data.element)
        }
    } else if let Some(action_fx) = &hitbox_action.action_fx {
        // Get from ActionFX.Emitter
        if let Some(emitter) = &action_fx.emitter {
            if let Some(anim_value) = &emitter.anim {
                // Try to extract AnimIndex from different structures
                if let Some(anim_index) = extract_anim_index_from_value(anim_value) {
                    leak_string(anim_index)
                } else {
                    get_default_projectile_effect(&data.element)
                }
            } else {
                get_default_projectile_effect(&data.element)
            }
        } else {
            get_default_projectile_effect(&data.element)
        }
    } else if let Some(emitter) = &hitbox_action.emitter {
        // AttackAction - get from direct Emitter (like cut.json, stomp.json)
        if let Some(anim_value) = &emitter.anim {
            // Try to extract AnimIndex from different structures
            if let Some(anim_index) = extract_anim_index_from_value(anim_value) {
                leak_string(anim_index)
            } else {
                get_default_projectile_effect(&data.element)
            }
        } else {
            get_default_projectile_effect(&data.element)
        }
    } else if let Some(tile_emitter) = &hitbox_action.tile_emitter {
        // DashAction - get from TileEmitter
        if let Some(anim_value) = &tile_emitter.anim {
            leak_string(
                anim_value
                    .get("AnimIndex")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            )
        } else {
            get_default_projectile_effect(&data.element)
        }
    } else {
        // For other action types, use element-based defaults
        get_default_projectile_effect(&data.element)
    };

    // Get hit effect from HitFX
    let hit_effect = if let Some(hit_fx) = &data.hit_fx {
        if let Some(emitter) = &hit_fx.emitter {
            if let Some(anim) = &emitter.anim {
                leak_string(
                    anim.get("AnimIndex")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                )
            } else if let Some(anims) = &emitter.anims {
                if let Some(first_anim) = anims.first() {
                    // Handle both wrapped and direct animation data
                    if let Some(anim) = &first_anim.anim {
                        leak_string(anim.anim_index.clone())
                    } else if let Some(anim_index) = &first_anim.anim_index {
                        leak_string(anim_index.clone())
                    } else {
                        get_default_hit_effect(&data.element)
                    }
                } else {
                    get_default_hit_effect(&data.element)
                }
            } else {
                get_default_hit_effect(&data.element)
            }
        } else {
            get_default_hit_effect(&data.element)
        }
    } else {
        // For moves without hit effects, use element-appropriate defaults
        get_default_hit_effect(&data.element)
    };

    // Cast effect based on element type
    let cast_effect = get_default_cast_effect(&data.element);

    Ok((projectile_effect, hit_effect, cast_effect))
}

fn get_default_projectile_effect(element: &str) -> &'static str {
    match element {
        "fire" => "Weather_Ball_Fire_2",
        "water" => "Water_Spout_Up",
        "grass" => "Vine_Whip_2",
        "electric" => "Charge_Beam_Shot",
        "normal" => "Vacuum_Wave_Fist",
        _ => "Vacuum_Wave_Fist",
    }
}

fn get_default_hit_effect(element: &str) -> &'static str {
    match element {
        "fire" => "Weather_Ball_Fire",
        "water" => "Water_Spout_Splash",
        "grass" => "Vine_Whip",
        "electric" => "Charge_Beam_Hit",
        "normal" => "Vacuum_Wave",
        _ => "Vacuum_Wave",
    }
}

fn get_default_cast_effect(element: &str) -> &'static str {
    match element {
        "fire" => "Stat_Red_Ring",
        "water" => "Stat_Blue_Ring",
        "grass" => "Stat_Green_Ring",
        "electric" => "Stat_Yellow_Ring",
        _ => "Stat_White_Ring",
    }
}

// Helper function to extract AnimIndex from different JSON structures
fn extract_anim_index_from_value(anim_value: &serde_json::Value) -> Option<String> {
    // Try direct AnimIndex (MoveToEmitter structure)
    if let Some(anim_index) = anim_value.get("AnimIndex") {
        if let Some(index_str) = anim_index.as_str() {
            return Some(index_str.to_string());
        }
    }

    // Try nested Anim.AnimIndex (SingleEmitter structure)
    if let Some(nested_anim) = anim_value.get("Anim") {
        if let Some(anim_index) = nested_anim.get("AnimIndex") {
            if let Some(index_str) = anim_index.as_str() {
                return Some(index_str.to_string());
            }
        }
    }

    None
}

// Helper function to convert String to &'static str by leaking memory
// This is needed because our Spell struct expects &'static str
fn leak_string(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
