use nalgebra::Point2;
use serde::{Deserialize, Serialize};

use crate::attack::{AttackConstructor, RecoverInfo};

#[derive(Debug, Deserialize, Serialize)]
pub struct NpcConstructor {
    pub name: String,
    pub close_melee_attack_distance: f32,
    pub close_melee_attacks: Vec<AttackConstructor>,
    pub melee_attack_distance: f32,
    pub melee_attacks: Vec<AttackConstructor>,
    pub ranged_attack_distance: f32,
    pub ranged_attacks: Vec<AttackConstructor>,
    pub hp: i32,
}

impl NpcConstructor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl Default for NpcConstructor {
    fn default() -> Self {
        NpcConstructor {
            name: String::new(),
            close_melee_attack_distance: 0.0,
            close_melee_attacks: Vec::new(),
            melee_attack_distance: 0.0,
            melee_attacks: Vec::new(),
            ranged_attack_distance: 0.0,
            ranged_attacks: Vec::new(),
            hp: 0,
        }
    }
}

pub struct NpcInfo {
    pub position: Point2<f32>,
    close_melee_attack_index: u8,
    pub close_melee_attack_distance: f32,
    pub close_melee_attacks: Vec<AttackConstructor>,
    melee_attack_index: u8,
    pub melee_attack_distance: f32,
    pub melee_attacks: Vec<AttackConstructor>,
    ranged_attack_index: u8,
    pub ranged_attack_distance: f32,
    pub ranged_attacks: Vec<AttackConstructor>,
    // pub attacking: Option<AttackView>,
    recovering: Option<RecoverInfo>,
    pub hp: i32,
    max_hp: i32,
}

impl NpcInfo {
    fn from_constructor(constructor: NpcConstructor, position: Point2<f32>) -> Self {
        let NpcConstructor {
            close_melee_attack_distance,
            close_melee_attacks,
            melee_attack_distance,
            melee_attacks,
            ranged_attack_distance,
            ranged_attacks,
            hp,
            ..
        } = constructor;
        NpcInfo {
            position,
            close_melee_attack_index: 0,
            close_melee_attack_distance,
            close_melee_attacks,
            melee_attack_index: 0,
            melee_attack_distance,
            melee_attacks,
            ranged_attack_index: 0,
            ranged_attack_distance,
            ranged_attacks,
            // attacking: None,
            recovering: None,
            hp,
            max_hp: hp,
        }
    }
}
