use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum KeyActionKind {
    Pressed,
    Released,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Join,
    Move(KeyActionKind, Move),
    HeroDash,
    HeroAttack,
    RequestFrameNumber,
}

impl Message {
    pub fn from_slice(data: &[u8]) -> Self {
        rmp_serde::from_slice(data).unwrap()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}
