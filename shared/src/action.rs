use serde::{Deserialize, Serialize};

use nalgebra::Point2;

use crate::attack::ComplexAttack;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Move {
    pub to_position: Point2<f32>,
    pub speed: f32,
}

impl Move {
    pub fn update(&mut self, _dt: u128) {
        // move the character in the direction of this point
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Action {
    Move(Move),
    Attack(ComplexAttack),
}

impl Action {
    pub fn update(&mut self, dt: u128) {
        match self {
            Action::Move(m) => m.update(dt),
            Action::Attack(attack) => attack.update(dt),
        }
    }
}
