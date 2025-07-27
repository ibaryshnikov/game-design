use nalgebra::Point2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CharacterSettings {
    pub dash_duration: u128,
    pub dash_distance: u128,
}

pub trait Character {
    fn receive_damage(&mut self);
    fn get_position(&self) -> Point2<f32>;
    fn get_size(&self) -> f32;
}
