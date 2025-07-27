use nalgebra::Point2;
use serde::{Deserialize, Serialize};

use shared::action::Action;

// Updates about Boss entities we send from
// the server to clients. Do not include attacks
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Boss {
    pub position: Point2<f32>,
    pub action: Action,
    pub hp: i32,
    pub max_hp: i32,
}
