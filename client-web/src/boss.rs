use web_sys::CanvasRenderingContext2d;

use game_core::boss::Boss;

use shared::action::Action;

use crate::attack::{AttackView /*ComplexAttackView*/};

pub struct BossView<'a> {
    pub boss_info: &'a Boss,
}

impl<'a> BossView<'a> {
    pub fn new(boss_info: &'a Boss) -> Self {
        Self { boss_info }
    }
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        self.draw_body(ctx);
        self.draw_attack(ctx);
        // self.draw_small_hp_bar(ctx);
        // self.draw_health_bar(ctx);
    }
    pub fn draw_body(&self, ctx: &CanvasRenderingContext2d) {
        let info = &self.boss_info;
        let x = info.position.x as f64;
        let y = info.position.y as f64;

        ctx.set_stroke_style_str("black");
        ctx.begin_path();
        let _ = ctx.arc(x, y, info.size as f64, 0.0, 2.0 * std::f64::consts::PI);
        ctx.stroke();
    }
    pub fn draw_small_hp_bar(&self, ctx: &CanvasRenderingContext2d) {
        let info = &self.boss_info;
        let x = info.position.x - info.size;
        let y = info.position.y - 50.0;
        let bar_width = 60.0;
        let bar_height = 8.0;
        ctx.set_fill_style_str("red");
        ctx.fill_rect(x as f64, y as f64, bar_width, bar_height);
        ctx.set_fill_style_str("green");
        // console_log!("boss hp {} max hp {}", info.hp, info.max_hp);
        // console_log!("boss hp left percent {}", info.hp_left_percent());
        let width = bar_width * info.hp_left_percent() as f64;
        ctx.fill_rect(x as f64, y as f64, width, bar_height);
    }
    pub fn draw_hp_bar(&self, ctx: &CanvasRenderingContext2d) {
        let info = self.boss_info;

        let x = 100.0;
        let y = 700.0;
        let bar_width = 800.0;
        let bar_height = 10.0;

        // draw red background
        ctx.set_fill_style_str("red");
        ctx.fill_rect(x, y, bar_width, bar_height);

        // draw hp left as green
        ctx.set_fill_style_str("green");
        let width = bar_width * info.hp_left_percent() as f64;
        ctx.fill_rect(x, y, width, bar_height);
    }
    pub fn draw_attack(&self, ctx: &CanvasRenderingContext2d) {
        if let Action::Attack(attack_info) = &self.boss_info.action {
            let attack_view = AttackView::new(attack_info);
            attack_view.draw(ctx);
        }
        // if let Some(attack) = &self.boss_info.attacking_complex {
        //     let attack_view = ComplexAttackView::new(attack);
        //     attack_view.draw(ctx);
        // }
    }
}
