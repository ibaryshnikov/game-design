use serde::{Deserialize, Serialize};

use crate::attack::RecoverInfo;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CharacterSettings {
    pub dash_duration: u128,
    pub dash_distance: u128,
}

pub trait Character {
    fn get_recovering_state(&mut self) -> &mut Option<RecoverInfo>;
    fn clear_recovering_state(&mut self);
    fn update_recovery(&mut self, dt: u128) {
        let Some(recover_info) = self.get_recovering_state() else {
            return;
        };
        recover_info.update(dt);
        if recover_info.completed() {
            self.clear_recovering_state();
        }
    }
}
