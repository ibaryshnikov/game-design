use nalgebra::Point2;

use crate::attack::ComplexAttack;

pub struct Move {
    pub to_position: Point2<f32>,
    pub speed: f32,
}

impl Move {
    pub fn update(&mut self) {
        // move the character in the direction of this point
    }
}

pub enum Action {
    Move(Move),
    Attack(ComplexAttack),
}

impl Action {
    pub fn update(&mut self) {
        match self {
            Action::Move(m) => m.update(),
            Action::Attack(attack) => attack.update(),
        }
    }
}
