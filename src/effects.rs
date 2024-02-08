use bevy::prelude::*;
use strum::{Display, EnumString};

use crate::{map::Position, vector2_int::Vector2Int, GameState};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn_test_effect);
    }
}

#[derive(Component, Debug)]
pub struct Effect {
    pub name: String,
    pub is_loop: bool,
}

#[derive(Component, Debug, Hash, PartialEq, Eq, EnumString, Display, Copy, Clone)]
enum _Effect {
    /// Dig
    #[strum(serialize = "0002")]
    _0002,
    /// (Multicolor rings) // NF
    #[strum(serialize = "0003")]
    _0003,
    /// (Floating rocks) // NF
    #[strum(serialize = "0004")]
    _0004,
    /// Dynamic Punch
    #[strum(serialize = "0005")]
    _0005,
    /// (Beam purple, impact blue) // NF
    #[strum(serialize = "0006")]
    _0006,
    /// Heal Bell
    #[strum(serialize = "0007")]
    _0007,
    /// Rage(000) / Octazooka(001=projectile) / Swagger(002)
    #[strum(serialize = "008")]
    _0008,
    /// (Fire Impact, Circle, Growing) // NF
    #[strum(serialize = "009")]
    _0009,
    /// Explosion
    #[strum(serialize = "0010")]
    _0010,
    /// Bonemerang / Bone Rush (000=projectile) (001=impact)
    #[strum(serialize = "0011")]
    _0011,
    /// Crush Claw / Dragon Claw / Metal Claw
    #[strum(serialize = "0012")]
    _0012,
    /// (Bubbles) // NF
    #[strum(serialize = "0013")]
    _0013,
    /// Sharpen
    #[strum(serialize = "0014")]
    _0014,
    /// Taunt
    #[strum(serialize = "0015")]
    _0015,
    /// Cleanse
    #[strum(serialize = "0016")]
    _0016,
    /// Feint?
    #[strum(serialize = "0017")]
    _0017,
    /// Shock Wave
    #[strum(serialize = "0018")]
    _0018,
    /// (Electric ball) // NF
    #[strum(serialize = "0019")]
    _0019,
    /// Lock-On(001) / Helping Hand(005) / (Circle Blue)
    #[strum(serialize = "0020")]
    _0020,
    /// Dive(000) / Ice Ball(001)
    #[strum(serialize = "0021")]
    _0021,
    /// Mud Sport() / Sludge(002)
    #[strum(serialize = "0022")]
    _0022,
    /// NF
    #[strum(serialize = "0023")]
    _0023,
    /// Drought
    #[strum(serialize = "0024")]
    _0024,
    /// Encore
    #[strum(serialize = "0025")]
    _0025,
    /// (Blue fang 000-0007) /// NF
    #[strum(serialize = "0026")]
    _0026,
    // Feather Dance
    #[strum(serialize = "0027")]
    _0027,
    /// Fire Blast(002) / Fire Punch (003) / Flame Wheel (002 / 003) / Flamethrower (005/003)
    #[strum(serialize = "0028")]
    _0028,
    /// Wil-O-Wisp(003)
    #[strum(serialize = "0029")]
    _0029,
    /// Fire Blast / Flame Wheel (003)
    #[strum(serialize = "0030")]
    _0030,
    /// (Impact Yellow) // NF
    #[strum(serialize = "0031")]
    _0031,
    /// Sunny Day
    #[strum(serialize = "0032")]
    _0032,
    /// Petal Dance(003)
    #[strum(serialize = "0033")]
    _0033,
    /// Eruption (001)
    #[strum(serialize = "0034")]
    _0034,
    /// (Feather)
    #[strum(serialize = "0035")]
    _0035,
    /// (Pilar of fire)
    #[strum(serialize = "0036")]
    _0036,
    /// (Yellow, Blue, Gathering)
    #[strum(serialize = "0037")]
    _0037,
    /// (Gray Stars, Yellow Circle, Electric)
    #[strum(serialize = "0039")]
    _0039,
    /// (Yellow Canon, Electric)
    #[strum(serialize = "0040")]
    _0040,
    /// (Smoke, Paper)
    #[strum(serialize = "0041")]
    _0041,
    /// Charge (SSJ, Electric, Charge)
    #[strum(serialize = "0042")]
    _0042,

    /// Rock Smash
    #[strum(serialize = "0110")]
    _0110,

    /// Shadow Sneak
    #[strum(serialize = "0184")]
    _0184,

    /// Charge Beam
    #[strum(serialize = "0215")]
    _0215,
}

fn spawn_test_effect(mut commands: Commands) {
    // commands.spawn((
    //     Name::new("TestEffect"),
    //     Effect {
    //         name: "Flame_Wheel".to_string(),
    //     },
    //     Position(Vector2Int::new(3, 3)),
    // ));
}
