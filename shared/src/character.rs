use crate::attack::RecoverInfo;

pub trait Character {
    fn get_recovering_state(&mut self) -> &mut Option<RecoverInfo>;
    fn clear_recovering_state(&mut self);
    fn update_recovery(&mut self) {
        let Some(recover_info) = self.get_recovering_state() else {
            return;
        };
        let time_passed = recover_info.started_at.elapsed().as_millis();
        if time_passed > recover_info.time_to_complete {
            self.clear_recovering_state();
        }
    }
}
