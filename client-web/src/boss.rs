use web_sys::CanvasRenderingContext2d;

use game_core::boss::Boss;

use crate::attack::{AttackView, /*ComplexAttackView*/};

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
        // self.draw_health_bar(ctx);
    }
    pub fn draw_body(&self, ctx: &CanvasRenderingContext2d) {
        let x = self.boss_info.position.x as f64;
        let y = self.boss_info.position.y as f64;

        ctx.set_stroke_style_str("black");
        ctx.begin_path();
        let _ = ctx.arc(x, y, 30.0, 0.0, 2.0 * std::f64::consts::PI);
        ctx.stroke();
    }
    // pub fn draw_health_bar(&self, ctx: &CanvasRenderingContext2d) {
    //     let start = iced_core::Point::new(100.0, 700.0);
    //     let bar_width = 800.0;
    //     let bar_height = 10.0;
    //
    //     // draw black background
    //     let path = Path::new(|b| {
    //         let size = Size::new(bar_width, bar_height);
    //         b.rectangle(start, size);
    //     });
    //
    //     ctx.fill(&path, Color::from_rgb8(0, 0, 0));
    //
    //     // draw hp left as green
    //     let path = Path::new(|b| {
    //         let width = bar_width * self.boss_info.hp_left_percent();
    //         let size = Size::new(width, bar_height);
    //         b.rectangle(start, size);
    //     });
    //
    //     ctx.fill(&path, Color::from_rgb8(255, 0, 0));
    // }
    pub fn draw_attack(&self, ctx: &CanvasRenderingContext2d) {
        if let Some(attack_info) = &self.boss_info.attacking {
            let attack_view = AttackView::new(attack_info);
            attack_view.draw(ctx);
        }
        // if let Some(attack) = &self.boss_info.attacking_complex {
        //     let attack_view = ComplexAttackView::new(attack);
        //     attack_view.draw(ctx);
        // }
    }
}
