use std::ops::{Range, RangeInclusive};

#[derive(Debug, Clone)]
pub struct ProjectileSpell {
    pub visual_effect: String,
    pub damage: i32,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    Projectile(ProjectileSpell),
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub range: RangeInclusive<i32>,
    pub spell_type: SpellType,
}
