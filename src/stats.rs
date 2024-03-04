use bevy::prelude::*;

const MAX_STAT: i32 = 255;
const MAX_HP: i32 = 999;

#[derive(Component, Debug)]
pub struct Stat {
    base: i32,
    bonus: i32,
}

impl Stat {
    pub fn new(base: i32) -> Self {
        Self { base, bonus: 0 }
    }

    pub fn value(&self) -> i32 {
        self.base + self.bonus
    }
}

#[derive(Component, Debug)]
pub struct Stats {
    pub health: Stat,
    pub attack: Stat,
    pub special_attack: Stat,
    pub defense: Stat,
    pub special_defense: Stat,
    pub speed: Stat,
}

impl Stats {
    pub fn is_dead(&self) -> bool {
        self.health.value() <= 0
    }
}
