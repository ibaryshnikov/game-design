use iced_widget::canvas::Frame;

use game_core::scene::Scene;

use crate::boss::BossView;
use crate::hero::HeroView;

pub struct SceneView<'a> {
    pub scene_info: &'a Scene,
}

impl<'a> SceneView<'a> {
    pub fn new(scene_info: &'a Scene) -> Self {
        Self { scene_info }
    }
    pub fn draw(&self, frame: &mut Frame) {
        let scene = &self.scene_info;
        for boss in scene.npc.iter() {
            BossView::new(boss).draw(frame);
        }
        for hero in scene.characters.values() {
            HeroView::new(hero).draw(frame);
        }
    }
}
