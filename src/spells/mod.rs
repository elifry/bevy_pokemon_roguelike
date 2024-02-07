#[derive(Debug, Clone)]
pub struct ProjectileSpell {
    pub visual_effect: String,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    Projectile(ProjectileSpell),
}

#[derive(Debug, Clone)]
pub struct Spell {
    pub name: String,
    pub spell_type: SpellType,
}
