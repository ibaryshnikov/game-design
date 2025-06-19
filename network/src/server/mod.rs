use serde::{Deserialize, Serialize};

pub mod boss;
pub mod hero;

use boss::Boss;
use hero::Hero;

// Updates to all npc on the current scene
#[derive(Debug, Deserialize, Serialize)]
pub struct NpcListUpdate {
    data: Vec<Boss>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterUpdate {
    id: u128,
    data: Hero,
}

// It's used to send server updates, no need for extra Box
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Deserialize, Serialize)]
pub enum Update {
    Character(CharacterUpdate),
    NpcList(NpcListUpdate),
    Projectile, // some updates related to projectiles
    Entity,     // some updates related to entities
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
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
