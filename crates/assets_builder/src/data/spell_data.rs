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
        anims: Vec<AnimEvent>,
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
        anims: Vec<AnimEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.StealItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StealItem {
        affect_target: bool,
        silent_check: bool,
        top_down: bool,
        message: EventMsg,
        priority_item: String,
        states: Vec<EventState>,
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
        anims: Vec<AnimEvent>,
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
        anims: Vec<AnimEvent>,
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
        anims: Vec<AnimEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.AddElementEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    AddElement { target_element: String },
    #[serde(rename = "PMDC.Dungeon.NatureElementEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    NatureElement {
        terrain_pair: HashMap<TerrainType, String>,
    },
    #[serde(rename = "PMDC.Dungeon.ChangeToAbilityEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ChangeToAbility {
        target_ability: String,
        affect_target: bool,
        silent_check: bool,
    },
    #[serde(rename = "PMDC.Dungeon.SwapStatsEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    SwapStats {
        #[serde(rename = "StatusIDs")]
        status_ids: Vec<String>,
    },
    #[serde(rename = "PMDC.Dungeon.DestroyItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    DestroyItem {
        top_down: bool,
        held_only: bool,
        priority_item: String,
        states: Vec<EventState>,
    },
    #[serde(rename = "PMDC.Dungeon.RemoveTrapEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveTrap,
    #[serde(rename = "PMDC.Dungeon.ThrowBackEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ThrowBack {
        distance: i64,
        hit_event: Box<BaseEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.MaxHPDamageEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    MaxHPDamage {
        #[serde(rename = "HPFraction")]
        hp_fraction: i64,
    },
    #[serde(rename = "PMDC.Dungeon.KnockBackEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    KnockBack { distance: i64 },
    #[serde(rename = "PMDC.Dungeon.ReflectStatsEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ReflectStats {
        #[serde(rename = "StatusIDs")]
        status_ids: Vec<String>,
    },
    #[serde(rename = "PMDC.Dungeon.TransferStatusEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    TransferStatus {
        remove: bool,
        major_status: bool,
        minor_status: bool,
        bad_status: bool,
        good_status: bool,
    },
    #[serde(rename = "PMDC.Dungeon.ReflectAbilityEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ReflectAbility { affect_target: bool, msg: EventMsg },
    #[serde(rename = "PMDC.Dungeon.RandomMoveEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RandomMove,
    #[serde(rename = "PMDC.Dungeon.StrongestMoveEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StrongestMove,
    #[serde(rename = "PMDC.Dungeon.RemoveItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveItem { blocked_by_terrain: bool },
    #[serde(rename = "PMDC.Dungeon.AdditionalEndEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    AdditionalEnd { base_events: Vec<BaseEvent> },
    #[serde(rename = "PMDC.Dungeon.InvokeCustomBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    InvokeCustomBattle {
        hitbox_action: HitboxAction,
        explosion: Explosion,
        new_data: RawData,
        msg: EventMsg,
        affect_target: bool,
    },
    #[serde(rename = "PMDC.Dungeon.RestoreHPEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RestoreHP {
        numerator: i32,
        denominator: i32,
        affect_target: bool,
    },
    #[serde(rename = "PMDC.Dungeon.EndeavorEvent, PMDC")]
    Endeavor,
    #[serde(rename = "PMDC.Dungeon.CutHPDamageEvent, PMDC")]
    CutHPDamage,
    #[serde(rename = "PMDC.Dungeon.LevelDamageEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    LevelDamage {
        affect_target: bool,
        numerator: i32,
        denominator: i32,
    },
    #[serde(rename = "PMDC.Dungeon.NatureSpecialEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    NatureSpecial {
        terrain_pair: HashMap<TerrainType, Box<BaseEvent>>,
        nature_pair: HashMap<Element, Box<BaseEvent>>,
    },
    #[serde(rename = "PMDC.Dungeon.SwitcherEvent, PMDC")]
    Switcher,
    #[serde(rename = "PMDC.Dungeon.DisableBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    DisableBattle {
        #[serde(rename = "LastSlotStatusID")]
        last_slot_status_id: String,
        random_fallback: bool,
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<AnimEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.MimicBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    MimicBattle {
        #[serde(rename = "LastMoveStatusID")]
        last_move_status_id: String,
        new_move_charges: i32,
    },
    #[serde(rename = "PMDC.Dungeon.ItemRestoreEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ItemRestore {
        held_only: bool,
        item_index: String,
        default_items: Vec<String>,
        success_msg: EventMsg,
    },
    #[serde(rename = "PMDC.Dungeon.BasePowerDamageEvent, PMDC")]
    BasePowerDamage,
    #[serde(rename = "PMDC.Dungeon.FutureAttackEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    FutureAttack {
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<AnimEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.DropItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    DropItem {
        message: EventMsg,
        silent_check: bool,
        top_down: bool,
        held_only: bool,
        priority_item: String,
        states: Vec<EventState>,
    },
    #[serde(rename = "PMDC.Dungeon.SpeedSwapEvent, PMDC")]
    SpeedSwap,
    #[serde(rename = "PMDC.Dungeon.UseFoeItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    UseFoeItem {
        affect_target: bool,
        silent_check: bool,
        top_down: bool,
        held_only: bool,
        priority_item: String,
        states: Vec<EventState>,
    },
    #[serde(rename = "PMDC.Dungeon.WeatherStackEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    WeatherStack {
        #[serde(rename = "WeatherID")]
        weather_id: String,
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<AnimEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.SketchBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    SketchBattle {
        #[serde(rename = "LastMoveStatusID")]
        last_move_status_id: String,
    },
    #[serde(rename = "PMDC.Dungeon.ChangeToElementEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ChangeToElement { target_element: String },
    #[serde(rename = "PMDC.Dungeon.AffectHighestStatBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    AffectHighestStatBattle {
        affect_target: bool,
        atk_stat: String,
        def_stat: String,
        sp_atk_stat: String,
        sp_def_stat: String,
        anonymous: bool,
        stack: i32,
    },
    #[serde(rename = "PMDC.Dungeon.RemoveStatusBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveStatusBattle {
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
    },
    #[serde(rename = "PMDC.Dungeon.StatusHPBattleEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StatusHPBattle {
        #[serde(rename = "HPFraction")]
        hp_fraction: i32,
        #[serde(rename = "StatusID")]
        status_id: String,
        affect_target: bool,
        silent_check: bool,
        anonymous: bool,
        trigger_msg: EventMsg,
        anims: Vec<AnimEvent>,
    },
    #[serde(rename = "PMDC.Dungeon.ReflectElementEvent, PMDC")]
    ReflectElement,
    #[serde(rename = "PMDC.Dungeon.PainSplitEvent, PMDC")]
    PainSplit,
    #[serde(rename = "PMDC.Dungeon.SpiteEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Spite {
        #[serde(rename = "LastSlotStatusID")]
        last_slot_status_id: String,
        #[serde(rename = "PP")]
        pp: i32,
    },
    #[serde(rename = "PMDC.Dungeon.OHKODamageEvent, PMDC")]
    OHKODamage,
    #[serde(rename = "PMDC.Dungeon.HopEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Hop { distance: i32, reverse: bool },
    #[serde(rename = "PMDC.Dungeon.RemoveTerrainEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveTerrain {
        tile_types: Vec<String>,
        remove_sound: String,
        remove_anim: Emitter,
    },
    #[serde(rename = "PMDC.Dungeon.SwitchHeldItemEvent, PMDC")]
    SwitchHeldItem,
    #[serde(rename = "PMDC.Dungeon.SetItemStickyEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    SetItemSticky {
        sticky: bool,
        top_down: bool,
        held_only: bool,
        priority_item: String,
        states: Vec<EventState>,
    },
    #[serde(rename = "PMDC.Dungeon.PowerTrickEvent, PMDC")]
    PowerTrick,
    #[serde(rename = "PMDC.Dungeon.BegItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    BegItem {
        top_down: bool,
        held_only: bool,
        priority_item: String,
        states: Vec<EventState>,
    },
    #[serde(rename = "PMDC.Dungeon.RestEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Rest {
        #[serde(rename = "SleepID")]
        sleep_id: String,
    },
    #[serde(rename = "PMDC.Dungeon.WarpAlliesInEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    WarpAlliesIn {
        distance: i64,
        amount: i64,
        farthest_first: bool,
        silent_fail: bool,
        msg: EventMsg,
    },
    #[serde(rename = "PMDC.Dungeon.RandomGroupWarpEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RandomGroupWarp { distance: i64, affect_target: bool },
    #[serde(rename = "PMDC.Dungeon.StatSplitEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    StatSplit { attack_stats: bool },
    #[serde(rename = "PMDC.Dungeon.KnockMoneyEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    KnockMoney { multiplier: i64 },
    #[serde(rename = "PMDC.Dungeon.TransformEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Transform {
        affect_target: bool,
        #[serde(rename = "StatusID")]
        status_id: String,
        transform_charges: i64,
    },
    #[serde(rename = "PMDC.Dungeon.PsywaveDamageEvent, PMDC")]
    PsywaveDamage,
    #[serde(rename = "PMDC.Dungeon.KnockItemEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    KnockItem {
        top_down: bool,
        held_only: bool,
        priority_item: String,
        states: Vec<EventState>,
    },
    #[serde(rename = "PMDC.Dungeon.UserHPDamageEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    UserHPDamage { reverse: bool },
    #[serde(rename = "PMDC.Dungeon.BestowItemEvent, PMDC")]
    BestowItem,
    #[serde(rename = "PMDC.Dungeon.ShatterTerrainEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    ShatterTerrain { tile_types: Vec<String> },
    #[serde(rename = "PMDC.Dungeon.GroupEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    Group { base_events: Vec<BaseEvent> },
    #[serde(rename = "PMDC.Dungeon.RemoveAbilityEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveAbility { target_ability: String },
    #[serde(rename = "PMDC.Dungeon.RemoveElementEvent, PMDC")]
    #[serde(rename_all = "PascalCase")]
    RemoveElement { target_element: String },
    #[serde(rename = "PMDC.Dungeon.SwapAbilityEvent, PMDC")]
    SwapAbility,
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
    #[serde(default)]
    pub grassy_terrain: Option<bool>,
    #[serde(default)]
    pub sunny: Option<bool>,
    #[serde(default)]
    pub rain: Option<bool>,
    #[serde(default)]
    pub sandstorm: Option<bool>,
    #[serde(default)]
    pub hail: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventMsg {
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimEvent {
    pub emitter: Emitter,
    pub sound: String,
    pub affect_target: bool,
    pub delay: i64,
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
    #[serde(rename = "PMDC.Dungeon.EdibleState")]
    Edible,
    #[serde(rename = "PMDC.Dungeon.ExtendWeatherState")]
    ExtendWeather,
    #[serde(rename = "PMDC.Dungeon.MajorStatusState")]
    MajorStatus,
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
    #[serde(rename = "RogueEssence.Content.SwingSwitchEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    SwingSwitch {
        loc_height: i64,
        #[serde(rename = "finished")]
        finished: bool,
        amount: i64,
        stream_time: i64,
        offset: i64,
        anim: Anim,
        rotation_time: i64,
        axis_ratio: f64,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.FiniteAreaEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FiniteArea {
        loc_height: i64,
        anims: Vec<Emitter>,
        range: i64,
        speed: i64,
        total_particles: i64,
        layer: usize,
    },
    #[serde(rename = "RogueEssence.Content.StaticAreaEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    StaticArea {
        loc_height: i64,
        anims: Vec<Emitter>,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        range: i64,
        layer: i64,
    },
    #[serde(rename = "RogueEssence.Content.FiniteReleaseRangeEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FiniteReleaseRange {
        loc_height: i64,
        range: i64,
        anims: Vec<Emitter>,
        speed: i64,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        start_distance: i64,
        layer: i64,
    },
    #[serde(rename = "RogueEssence.Content.AttachAreaEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    AttachArea {
        anims: Vec<Emitter>,
        range: i64,
        particles_per_burst: i64,
        add_height: i64,
        burst_time: i64,
        layer: i64,
    },
    #[serde(rename = "RogueEssence.Content.ColumnAnim, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Column {
        anim: Anim,
        total_time: i32,
        cycles: i32,
    },
    #[serde(rename = "RogueEssence.Content.FiniteGatherEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FiniteGather {
        loc_height: i64,
        anims: Vec<Anim>,
        use_dest: bool,
        travel_time: i64,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        start_distance: i64,
        end_distance: i64,
        start_variance: i64,
        layer: i64,
        cycles: i64,
    },
    #[serde(rename = "RogueEssence.Content.SqueezedAreaEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    SqueezedArea {
        loc_height: i64,
        anims: Vec<Emitter>,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        range: i64,
        height_speed: i64,
        speed_diff: i64,
        start_height: i64,
        height_diff: i64,
        layer: i64,
    },
    #[serde(rename = "RogueEssence.Content.FiniteSprinkleEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    FiniteSprinkle {
        loc_height: i64,
        anims: Vec<Emitter>,
        range: i64,
        speed: i64,
        total_particles: i64,
        height_speed: i64,
        speed_diff: i64,
        start_height: i64,
        layer: i64,
    },
    #[serde(rename = "RogueEssence.Content.AfterImageEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    AfterImage {
        anim_time: i32,
        burst_time: i32,
        alpha: i32,
        alpha_speed: i32,
    },
    #[serde(rename = "RogueEssence.Content.ClampEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Clamp {
        loc_height: i64,
        #[serde(rename = "finished")]
        finished: bool,
        anim1: Anim,
        anim2: Anim,
        offset: i64,
        half_offset: Offset,
        half_height: i64,
        linger_start: i64,
        move_time: i64,
        linger_end: i64,
    },
    #[serde(rename = "RogueEssence.Content.VortexEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    Vortex {
        loc_height: i64,
        anims: Vec<Anim>,
        bursts: i64,
        particles_per_burst: i64,
        burst_time: i64,
        range: i64,
        start_height: i64,
        end_height: i64,
        height_speed: i64,
        cycle_speed: i64,
    },
    #[serde(rename = "RogueEssence.Content.AttachReleaseRangeEmitter, RogueEssence")]
    #[serde(rename_all = "PascalCase")]
    AttachReleaseRange {
        range: i64,
        anims: Vec<Emitter>,
        speed: i64,
        particles_per_burst: i64,
        burst_time: i64,
        start_distance: i64,
        layer: i64,
    },
}
