use std::fmt::{self, Display};

use nalgebra::Point2;
use serde::{Deserialize, Serialize};

use crate::attack::{AttackConstructor, ComplexAttackConstructor, RecoverInfo};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NpcConstructor {
    pub name: String,
    pub attacks: Vec<NpcAttackInfo>,
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

pub struct NpcInfo {
    pub position: Point2<f32>,
    pub attacks: Vec<AttackConstructor>,
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

fn load_complex_attack_by_id(id: u32) -> ComplexAttackConstructor {
    let file_path = format!("../data/attack/complex_attack_{id}.json");
    let contents =
        std::fs::read(file_path).expect("Should read ComplexAttackConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode ComplexAttackConstructor")
}

pub fn load_complex_attacks(attack_info: Vec<NpcAttackInfo>) -> Vec<ComplexAttackConstructor> {
    attack_info
        .into_iter()
        .map(|item| load_complex_attack_by_id(item.id))
        .collect()
}

impl NpcInfo {
    fn from_constructor(constructor: NpcConstructor, position: Point2<f32>) -> Self {
        let NpcConstructor { attacks, hp, .. } = constructor;
        let attacks = load_attacks(attacks);
        NpcInfo {
            position,
            attacks,
            // attacking: None,
            recovering: None,
            hp,
            max_hp: hp,
        }
    }
}
