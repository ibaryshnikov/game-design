use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

use shared::action::Action;
use shared::character::CharacterSettings;
use shared::hero::Moving;

// Updates about Hero we send from the server
// to clients
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hero {
    pub id: u128,
    pub hp: i32,
    pub max_hp: i32,
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub moving: Moving,
    pub melee_attack_distance: f32,
    pub ranged_attack_distance: f32,
    pub action: Action,
    pub character_settings: CharacterSettings,
}
