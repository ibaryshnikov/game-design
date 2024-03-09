use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerAction {
    CharacterAction(u128, CharacterAction),
    NpcAction(u128, NpcAction),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CharacterAction {
    Move,
    Update, // update all stats like hp, mp, position, etc
    UpdatePosition,
    Dash,
    Attack,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NpcAction {
    Move,
    Update, // update all stats like hp, mp, position, etc
    UpdatePosition,
    Attack,
}
