use std::time::Instant;

use nalgebra::{Point2, Vector2};

use crate::moves::Moving;

pub struct Npc {
    pub id: String,
    pub hp: i32,
    max_hp: i32,
    pub position: Point2<f32>,
    direction: Vector2<f32>,
    moving: Moving,
    last_tick: Instant,
}
