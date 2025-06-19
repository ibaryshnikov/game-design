use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::boss::Boss;
use crate::hero::Hero;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scene {
    pub characters: HashMap<u128, Hero>,
    pub npc: Vec<Boss>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ServerMessage {
    Test,
    Scene(Scene),
}

impl ServerMessage {
    pub fn from_slice(data: &[u8]) -> Self {
        rmp_serde::from_slice(data).unwrap()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}
