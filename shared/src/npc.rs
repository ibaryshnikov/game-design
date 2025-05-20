use std::fmt::{self, Display};

use nalgebra::Point2;
use serde::{Deserialize, Serialize};

use crate::attack::{AttackConstructor, RecoverInfo};

#[derive(Debug, Deserialize, Serialize)]
pub struct NpcConstructor {
    pub name: String,
    pub close_melee_attack_distance: f32,
    pub close_melee_attacks: Vec<NpcAttackInfo>,
    pub melee_attack_distance: f32,
    pub melee_attacks: Vec<NpcAttackInfo>,
    pub ranged_attack_distance: f32,
    pub ranged_attacks: Vec<NpcAttackInfo>,
    pub hp: i32,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct NpcAttackInfo {
    pub id: u32,
    pub name: String,
}

impl Display for NpcAttackInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
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

fn load_attack_by_id(id: u32) -> AttackConstructor {
    let file_path = format!("../data/attack/attack_{id}.json");
    let contents = std::fs::read(file_path).expect("Should read AttackConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode AttackConstructor")
}

pub fn load_attacks(attack_info: Vec<NpcAttackInfo>) -> Vec<AttackConstructor> {
    attack_info
        .into_iter()
        .map(|item| load_attack_by_id(item.id))
        .collect()
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
        let close_melee_attacks = load_attacks(close_melee_attacks);
        let melee_attacks = load_attacks(melee_attacks);
        let ranged_attacks = load_attacks(ranged_attacks);
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
