use serde::{Deserialize, Serialize};

use nalgebra::Point2;

use crate::attack::{AttackInfo, ComplexAttack, RecoverInfo};
use crate::hero::{DashCooldown, DashInfo};

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
    Dash(DashInfo),
    DashCooldown(DashCooldown),
    Attack(AttackInfo),
    Recovery(RecoverInfo),
    ComplexAttack(ComplexAttack),
    Animation,
    Empty,
}

impl Action {
    pub fn is_some(&self) -> bool {
        !self.is_empty()
    }
    pub fn is_empty(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        if let Action::Empty = self {
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        *self = Action::Empty;
    }
}

impl Action {
    pub fn update(&mut self, dt: u128) {
        match self {
            Action::Move(m) => m.update(dt),
            Action::Dash(d) => d.update(dt),
            Action::DashCooldown(cooldown) => {
                cooldown.update(dt);
                if cooldown.completed() {
                    self.clear();
                }
            }
            Action::Attack(_attack) => {
                // attack.update(dt);
                // do nothing
            }
            Action::Recovery(recovery) => {
                recovery.update(dt);
                if recovery.completed() {
                    self.clear();
                }
            }
            Action::ComplexAttack(_attack) => {
                // attack.update(dt);
                // do nothing
            }
            Action::Animation => (), // do nothing
            Action::Empty => (),     // do nothing
        }
    }
}
