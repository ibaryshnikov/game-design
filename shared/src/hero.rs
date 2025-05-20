use std::time::Instant;

use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Moving {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}
#[derive(Debug)]
pub struct DashInfo {
    pub direction: Vector2<f32>,
    pub started: Instant,
    pub time_to_complete: u128,
}
impl DashInfo {
    pub fn percent_completed(&self) -> f32 {
        self.started.elapsed().as_millis() as f32 / self.time_to_complete as f32
    }
}
#[derive(Debug)]
pub struct DashCooldown {
    pub started: Instant,
    pub duration: u128,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hero {
    id: String,
    hp: i32,
    max_hp: i32,
    position: Vector2<f32>,
    direction: Vector2<f32>,
    moving: Moving,
}
