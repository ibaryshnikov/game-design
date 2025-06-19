use nalgebra::Point2;
use serde::{Deserialize, Serialize};

use shared::action::Action;
use shared::attack::{AttackInfo, ComplexAttack, RecoverInfo};

// Updates about Boss entities we send from
// the server to clients. Do not include attacks
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Boss {
    pub position: Point2<f32>,
    pub attacking: Option<AttackInfo>,
    pub recovering: Option<RecoverInfo>,
    pub attacking_complex: Option<ComplexAttack>,
    pub action: Option<Action>,
    pub hp: i32,
    pub max_hp: i32,
}
