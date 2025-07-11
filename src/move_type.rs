/// Represents the category of a move in Pokémon
/// This determines which stats are used in damage calculation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveCategory {
    /// Uses Attack and Defense stats for damage calculation
    Physical,
    /// Uses Special Attack and Special Defense stats for damage calculation
    Special,
    /// Doesn't deal direct damage, but may have other effects
    Status,
}

impl MoveCategory {
    /// Returns the attacking and defending stats used for damage calculation
    pub fn get_damage_stats(&self) -> Option<(AttackStat, DefenseStat)> {
        match self {
            MoveCategory::Physical => Some((AttackStat::Attack, DefenseStat::Defense)),
            MoveCategory::Special => Some((AttackStat::SpecialAttack, DefenseStat::SpecialDefense)),
            MoveCategory::Status => None,
        }
    }
}

/// The attacking stat used in damage calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackStat {
    Attack,
    SpecialAttack,
}

impl AttackStat {
    pub fn as_str(&self) -> &'static str {
        match self {
            AttackStat::Attack => "Attack",
            AttackStat::SpecialAttack => "Special Attack",
        }
    }
}

/// The defending stat used in damage calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseStat {
    Defense,
    SpecialDefense,
}

impl DefenseStat {
    pub fn as_str(&self) -> &'static str {
        match self {
            DefenseStat::Defense => "Defense",
            DefenseStat::SpecialDefense => "Special Defense",
        }
    }
}
