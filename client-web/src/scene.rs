use web_sys::CanvasRenderingContext2d;

use game_core::scene::Scene;

use crate::boss::BossView;
use crate::hero::HeroView;

// const DRAW_SMALL_HP_BAR: bool = true;
pub const DRAW_LARGE_HP_BAR: bool = true;

pub struct SceneView<'a> {
    pub scene_info: &'a Scene,
}

impl<'a> SceneView<'a> {
    pub fn new(scene_info: &'a Scene) -> Self {
        Self { scene_info }
    }
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, self_id: u128) {
        let scene = &self.scene_info;
        for boss in scene.npc.iter() {
            // console_log!("boss hp {} max {}", boss.hp, boss.max_hp);
            // console_log!("boss hp left percent {}", boss.hp_left_percent());
            BossView::new(boss).draw(ctx);
            if DRAW_LARGE_HP_BAR {
                BossView::new(boss).draw_hp_bar(ctx);
            } else {
                BossView::new(boss).draw_small_hp_bar(ctx);
            }
        }
        for hero in scene.characters.values() {
            if hero.id == self_id {
                continue;
            }
            let view = HeroView::new(hero);
            view.draw(ctx);
            view.draw_small_hp_bar(ctx);
        }
    }
}
