use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Moving {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashInfo {
    pub direction: Vector2<f32>,
    pub time_passed: u128,
    pub time_to_complete: u128,
}
impl DashInfo {
    pub fn new(direction: Vector2<f32>, time_to_complete: u128) -> Self {
        Self {
            direction,
            time_passed: 0,
            time_to_complete,
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
    }
    pub fn percent_completed(&self) -> f32 {
        self.time_passed as f32 / self.time_to_complete as f32
    }
    pub fn completed(&self) -> bool {
        self.percent_completed() >= 1.0
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashCooldown {
    pub time_passed: u128,
    pub duration: u128,
}

impl DashCooldown {
    pub fn new(duration: u128) -> Self {
        Self {
            time_passed: 0,
            duration,
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
    }
    pub fn completed(&self) -> bool {
        self.time_passed >= self.duration
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hero {
    id: String,
    hp: i32,
    max_hp: i32,
    position: Vector2<f32>,
    direction: Vector2<f32>,
    moving: Moving,
}
