use std::ops::RangeInclusive;

use crate::graphics::anim_data::AnimKey;

#[derive(Debug, Clone)]
pub struct ProjectileSpell {
    pub visual_effect: String,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    Projectile(ProjectileSpell),
}

#[derive(Debug, Clone)]
pub struct SpellHit {
    pub visual_effect: String,
    pub damage: i32,
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub range: RangeInclusive<i32>,
    pub spell_type: SpellType,
    pub hit: SpellHit,
    pub cast_animation: AnimKey,
}
