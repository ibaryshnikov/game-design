use iced_widget::canvas::Frame;

use game_core::scene::Scene;

use crate::boss::BossView;
use crate::hero::HeroView;

pub const DRAW_LARGE_HP_BAR: bool = true;

pub struct SceneView<'a> {
    pub scene_info: &'a Scene,
}

impl<'a> SceneView<'a> {
    pub fn new(scene_info: &'a Scene) -> Self {
        Self { scene_info }
    }
    pub fn draw(&self, frame: &mut Frame, self_id: u128) {
        let scene = &self.scene_info;
        for boss in scene.npc.iter() {
            let view = BossView::new(boss);
            view.draw(frame);
            if DRAW_LARGE_HP_BAR {
                view.draw_hp_bar(frame);
            } else {
                view.draw_small_hp_bar(frame);
            }
        }
        for hero in scene.characters.values() {
            if hero.id == self_id {
                continue;
            }
            let view = HeroView::new(hero);
            view.draw(frame);
            view.draw_small_hp_bar(frame);
        }
    }
}
