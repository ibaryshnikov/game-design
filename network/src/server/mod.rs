use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod boss;
pub mod hero;

pub use boss::Boss;
pub use hero::Hero;

// Updates to all npc on the current scene
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NpcListUpdate {
    data: Vec<Boss>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterUpdate {
    id: u128,
    data: Hero,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scene {
    pub frame_number: u128,
    pub characters: HashMap<u128, Hero>,
    pub npc: Vec<Boss>,
}

// It's used to send server updates, no need for extra Box
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Update {
    Scene(Scene), // update the whole scene
    Character(CharacterUpdate),
    NpcList(NpcListUpdate),
    Projectile, // some updates related to projectiles
    Entity,     // some updates related to entities
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    Test,
    SetId(u128),
    Update(Update),
}

impl Message {
    pub fn from_slice(data: &[u8]) -> Self {
        rmp_serde::from_slice(data).unwrap()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}
