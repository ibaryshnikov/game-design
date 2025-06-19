use serde::{Deserialize, Serialize};

use crate::hero::Hero;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CharacterAction {
    Move(KeyActionKind, Move),
    Dash,
    Attack,
    Update(Hero),
}

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    Join,
    CharacterAction(CharacterAction),
}


impl ClientMessage {
    pub fn from_slice(data: &[u8]) -> Self {
        rmp_serde::from_slice(data).unwrap()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}
