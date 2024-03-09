use std::collections::HashMap;

use common::element::Element;
use serde::{Deserialize, Serialize};

use super::common_data::RawTextData;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawSpellData {
    pub version: String,
    pub object: RawSpellObject,
}

impl RawSpellData {
    pub fn parse_from_json(spell_data: &[u8]) -> Result<RawSpellData, serde_json::Error> {
        let font_data: RawSpellData = serde_json::from_reader(spell_data)?;
        Ok(font_data)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawSpellObject {
    #[serde(rename = "$type")]
    pub object_type: String,
    pub name: RawTextData,
    pub desc: RawTextData,
    pub released: bool,
    pub comment: String,
    pub index_num: u32,
    pub base_charges: i32,
    pub strikes: i32,
    pub hitbox_action: HitboxAction,
    pub explosion: Explosion,
    pub data: RawData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawData {
    pub element: String,
    pub category: i32,
    pub hit_rate: i32,
    pub skill_states: Vec<SkillState>,
    pub before_try_actions: Vec<Option<serde_json::Value>>,
    pub before_actions: Vec<Option<serde_json::Value>>,
    pub on_actions: Vec<Option<serde_json::Value>>,
    pub before_explosions: Vec<Option<serde_json::Value>>,
    pub before_hits: Vec<Option<serde_json::Value>>,
    pub on_hits: Vec<EventKeyValue>,
    pub on_hit_tiles: Vec<EventKeyValue>,
    pub after_actions: Vec<Option<serde_json::Value>>,
    pub element_effects: Vec<Option<serde_json::Value>>,
    #[serde(rename = "IntroFX")]
    pub intro_fx: Vec<Fx>,
    #[serde(rename = "HitFX")]
    pub hit_fx: Fx,
    pub hit_char_action: CharAnim,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum CharAnim {
    #[serde(rename = "RogueEssence.Dungeon.CharAnimFrameType, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FrameType { action_type: i32 },
    #[serde(rename = "RogueEssence.Dungeon.CharAnimProcess, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Process { process: i32, anim_override: i32 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Anim {
    pub anim_index: String,
    pub frame_time: i32,
    pub start_frame: i32,
    pub end_frame: i32,
    pub anim_dir: i32,
    pub alpha: i32,
    pub anim_flip: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScreenMovement {
    pub min_shake: i32,
    pub max_shake: i32,
    pub max_shake_time: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventKeyValue {
    pub key: EventKey,
    pub value: BaseEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventKey {
    pub str: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum BaseEvent {
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "PMDC.Dungeon.GiveMapStatusEvent, PMDC")]
    GiveMapStatus {
        #[serde(rename = "StatusID")]
        status_id: String,
        counter: i64,
        msg_override: EventMsg,
        states: Vec<EventState>,
    },
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "PMDC.Dungeon.OnHitEvent, PMDC")]
    OnHit {
        base_events: Vec<BaseEvent>,
        require_damage: bool,
        require_contact: bool,
        chance: i64,
    },
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "PMDC.Dungeon.AdditionalEvent, PMDC")]
    Additional { base_events: Vec<BaseEvent> },
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "PMDC.Dungeon.DamageFormulaEvent, PMDC")]
    DamageFormula,
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "PMDC.Dungeon.NatureMoveEvent, PMDC")]
    NatureMove {
        terrain_pair: HashMap<TerrainType, String>,
        nature_pair: HashMap<Element, String>,
    },
    #[serde(rename = "PMDC.Dungeon.RemoveStateStatusBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveStateStatusBattle {
        states: Vec<EventState>,
        affect_target: bool,
        msg: EventMsg,
        anims: Vec<Emitter>,
    },
    #[serde(rename = "PMDC.Dungeon.StatusStackBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StatusStackBattle {
        stack: i64,
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<Emitter>,
    },
    #[serde(rename = "PMDC.Dungeon.StatusBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StatusBattle {
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<Emitter>,
    },
    #[serde(rename = "PMDC.Dungeon.StatusStateBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StatusStateBattle {
        states: Vec<EventState>,
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<Emitter>,
    },
    #[serde(rename = "PMDC.Dungeon.MirrorMoveEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    MirrorMove {
        #[serde(rename = "MoveStatusID")]
        move_status_id: String,
    },
    #[serde(rename = "PMDC.Dungeon.WeatherHPEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    WeatherHP {
        weather_pair: WeatherPair,
        #[serde(rename = "HPDiv")]
        hp_div: i64,
    },
    #[serde(rename = "PMDC.Dungeon.SetTrapEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    SetTrap {
        #[serde(rename = "TrapID")]
        trap_id: String,
    },
    #[serde(rename = "PMDC.Dungeon.RemoveTerrainStateEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveTerrainState {
        states: Vec<EventState>,
        remove_sound: String,
        remove_anim: Emitter,
    },
    #[serde(rename = "PMDC.Dungeon.GiveContinuousDamageEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    GiveContinuousDamage {
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<Emitter>,
    },
    #[serde(rename = "PMDC.Dungeon.AddElementEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    AddElement { target_element: String },
    #[serde(rename = "PMDC.Dungeon.NatureElementEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    NatureElement {
        terrain_pair: HashMap<TerrainType, String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum TerrainType {
    #[serde(rename = "electric_terrain")]
    Electric,
    #[serde(rename = "grassy_terrain")]
    Grassy,
    #[serde(rename = "misty_terrain")]
    Misty,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherPair {
    pub grassy_terrain: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventMsg {
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EventState {
    #[serde(rename = "PMDC.Dungeon.StatChangeState")]
    StatChange,
    #[serde(rename = "PMDC.Dungeon.WaterTerrainState")]
    WaterTerrain,
    #[serde(rename = "PMDC.Dungeon.LavaTerrainState")]
    LavaTerrain,
    #[serde(rename = "PMDC.Dungeon.AbyssTerrainState")]
    AbyssTerrain,
    #[serde(rename = "PMDC.Dungeon.WallTerrainState")]
    WallTerrain,
    #[serde(rename = "PMDC.Dungeon.FoliageTerrainState")]
    FoliageTerrain,
    #[serde(rename = "RogueEssence.Dungeon.CountDownState, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    CountDown { counter: i32 },
    #[serde(rename = "PMDC.Dungeon.BadStatusState")]
    BadStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkillState {
    #[serde(rename = "$type")]
    pub skill_state_type: String,
    pub power: Option<i32>,
    pub effect_chance: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Explosion {
    pub target_alignments: i32,
    pub hit_tiles: bool,
    pub range: i32,
    pub speed: i32,
    pub tile_emitter: Emitter,
    pub emitter: Emitter,
    #[serde(rename = "IntroFX")]
    pub intro_fx: Vec<Option<serde_json::Value>>,
    #[serde(rename = "ExplodeFX")]
    pub explode_fx: Fx,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fx {
    pub delay: i32,
    pub sound: String,
    pub emitter: Emitter,
    pub screen_movement: ScreenMovement,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum HitboxAction {
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "RogueEssence.Dungeon.ProjectileAction, RogueEssence")]
    Projectile {
        anim: Anim,
        emitter: Emitter,
        stream_emitter: Emitter,
        rays: i32,
        speed: i32,
        boomerang: bool,
        item_sprite: String,
        char_anim_data: CharAnim,
        hit_tiles: bool,
        range: i32,
        stop_at_hit: bool,
        stop_at_wall: bool,
        target_alignments: u32,
        tile_emitter: Emitter,
        pre_actions: Vec<Option<serde_json::Value>>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename_all = "PascalCase")]
    #[serde(rename = "RogueEssence.Dungeon.DashAction, RogueEssence")]
    Dash {
        anim: Anim,
        anim_offset: i32,
        emitter: Emitter,
        wide_angle: i32,
        snap_back: bool,
        char_anim: i32,
        appearance_mod: i32,
        hit_tiles: bool,
        range: i32,
        stop_at_hit: bool,
        stop_at_wall: bool,
        target_alignments: u32,
        tile_emitter: Emitter,
        pre_actions: Vec<Option<serde_json::Value>>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename = "RogueEssence.Dungeon.AttackAction, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Attack {
        hit_tiles: bool,
        burst_tiles: u32,
        emitter: Emitter,
        wide_angle: u32,
        char_anim_data: CharAnim,
        target_alignments: u32,
        tile_emitter: Emitter,
        pre_actions: Vec<Option<serde_json::Value>>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename = "RogueEssence.Dungeon.SelfAction, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    SelfAction {
        char_anim_data: CharAnim,
        target_alignments: u32,
        tile_emitter: Emitter,
        pre_actions: Vec<Fx>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename = "RogueEssence.Dungeon.AreaAction, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Area {
        emitter: Emitter,
        hit_tiles: bool,
        burst_tiles: u32,
        hit_area: usize,
        range: i32,
        speed: i32,
        char_anim_data: CharAnim,
        target_alignments: i32,
        tile_emitter: Emitter,
        pre_actions: Vec<Option<serde_json::Value>>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename = "RogueEssence.Dungeon.WaveMotionAction, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    WaveMotion {
        anim: Anim,
        wide: bool,
        speed: i32,
        linger: i32,
        char_anim_data: CharAnim,
        hit_tiles: bool,
        range: i32,
        stop_at_hit: bool,
        stop_at_wall: bool,
        target_alignments: u32,
        tile_emitter: Emitter,
        pre_actions: Vec<Fx>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename = "RogueEssence.Dungeon.ThrowAction, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Throw {
        anim: Anim,
        emitter: Emitter,
        range: i64,
        speed: i64,
        coverage: i64,
        item_sprite: String,
        char_anim_data: CharAnim,
        target_alignments: i64,
        tile_emitter: Emitter,
        pre_actions: Vec<Fx>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
    #[serde(rename = "RogueEssence.Dungeon.OffsetAction, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Offset {
        emitter: Emitter,
        hit_tiles: bool,
        burst_tiles: i64,
        hit_area: i64,
        range: i64,
        speed: i64,
        char_anim_data: CharAnim,
        target_alignments: i32,
        tile_emitter: Emitter,
        pre_actions: Vec<Fx>,
        #[serde(rename = "ActionFX")]
        action_fx: Fx,
        lag_behind_time: i32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum Emitter {
    #[serde(rename = "RogueEssence.Content.SingleEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Single {
        loc_height: i32,
        #[serde(rename = "finished")]
        finished: bool,
        offset: i32,
        anim: Box<Emitter>,
        layer: i32,
        use_dest: bool,
    },
    #[serde(rename = "RogueEssence.Content.StreamEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Stream {
        loc_height: i32,
        anims: Vec<Emitter>,
        shots: i32,
        burst_time: i32,
        start_distance: i32,
        end_diff: i32,
        layer: i32,
    },
    #[serde(rename = "RogueEssence.Content.BetweenEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Between {
        loc_height: i32,
        #[serde(rename = "finished")]
        finished: bool,
        anim_back: Box<Emitter>,
        anim_front: Box<Emitter>,
        height_back: i32,
        height_front: i32,
        offset: i32,
    },
    #[serde(rename = "RogueEssence.Content.AttachReleaseEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    AttachRelease {
        anims: Vec<Emitter>,
        speed: i32,
        particles_per_burst: u32,
        burst_time: i32,
        start_distance: i32,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.MoveToEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    MoveTo {
        loc_height: i32,
        #[serde(rename = "finished")]
        finished: bool,
        anim: Anim,
        offset_start: Offset,
        offset_end: Offset,
        height_start: i32,
        height_end: i32,
        linger_start: i32,
        linger_end: i32,
        move_time: i32,
        result_anim: Box<Emitter>,
        result_layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.RepeatEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Repeat {
        loc_height: i32,
        anim: Box<Emitter>,
        bursts: i32,
        burst_time: i32,
        layer: usize,
        offset: i32,
    },
    #[serde(rename = "RogueEssence.Content.CircleSquareSprinkleEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    CircleSquareSprinkle {
        loc_height: i32,
        anims: Vec<Emitter>,
        particles_per_tile: f32,
        height_speed: i64,
        speed_diff: i64,
        start_height: i64,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.FiniteReleaseEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FiniteRelease {
        loc_height: i32,
        anims: Vec<Emitter>,
        speed: i64,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        start_distance: i64,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.CircleSquareReleaseEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    CircleSquareRelease {
        loc_height: i32,
        anims: Vec<Emitter>,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        start_distance: i64,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.FiniteOverlayEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FiniteOverlay {
        loc_height: i32,
        #[serde(rename = "finished")]
        finished: bool,
        offset: i64,
        anim: Anim,
        movement: Offset,
        repeat_x: bool,
        repeat_y: bool,
        fade_in: i64,
        fade_out: i64,
        total_time: i64,
        layer: i64,
        color: String,
    },
    #[serde(rename = "RogueEssence.Content.MultiCircleSquareEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    MultiCircleSquare {
        loc_height: i64,
        #[serde(rename = "finished")]
        finished: bool,
        emitters: Vec<Emitter>,
    },
    #[serde(rename = "RogueEssence.Content.EmptyAttachEmitter, RogueEssence")]
    EmptyAttach,
    #[serde(rename = "RogueEssence.Content.EmptyFiniteEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    EmptyFinite { loc_height: i32 },
    #[serde(rename = "RogueEssence.Content.EmptyCircleSquareEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    EmptyCircleSquare { loc_height: i32 },
    #[serde(rename = "RogueEssence.Content.CircleSquareAreaEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    CircleArea {
        loc_height: i32,
        anims: Vec<Emitter>,
        particles_per_tile: f32,
        range_diff: i32,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.CircleSquareFountainEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    CircleSquareFountain {
        loc_height: i32,
        anims: Vec<Emitter>,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        start_distance: i64,
        range_diff: i64,
        height_ratio: f64,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.EmptyShootingEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    EmptyShooting { loc_height: i32 },
    #[serde(rename = "RogueEssence.Content.StaticAnim, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Static {
        anim: Anim,
        total_time: i32,
        cycles: i32,
        frame_offset: i32,
    },
    #[serde(rename = "RogueEssence.Content.EmittingAnim, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Emitting {
        result_anim: Box<Emitter>,
        layer: i64,
        anim: Anim,
        total_time: i64,
        cycles: i64,
        frame_offset: i64,
    },
    #[serde(rename = "RogueEssence.Content.ParticleAnim, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Particle {
        anim: Anim,
        total_time: i64,
        cycles: i64,
        frame_offset: i64,
    },
}
