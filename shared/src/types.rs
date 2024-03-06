use serde::{Deserialize, Serialize};

use crate::hero::Hero;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    MoveKeyUp(Move),
    MoveKeyDown(Move),
    HeroDash,
    HeroAttack,
    Hero(Hero),
}

impl Message {
    pub fn from_slice(data: &[u8]) -> Self {
        rmp_serde::from_slice(data).unwrap()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}
