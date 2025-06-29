use web_sys::CanvasRenderingContext2d;

use game_core::hero::Hero;

use crate::attack::AttackView;

pub struct HeroView<'a> {
    pub hero_info: &'a Hero,
}

impl<'a> HeroView<'a> {
    pub fn new(hero_info: &'a Hero) -> Self {
        Self { hero_info }
    }
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style_str("black");
        self.draw_body(ctx);
        self.draw_direction(ctx);
        self.draw_attack(ctx);
        self.draw_health_bar(ctx);
    }
    fn draw_body(&self, ctx: &CanvasRenderingContext2d) {
        if let Some(dash_info) = &self.hero_info.dashing {
            let percent_completed = dash_info.percent_completed();
            // console_log!("percent_completed is {percent_completed}");
            let x = self.hero_info.position.x + dash_info.direction.x * 150.0 * percent_completed;
            let y = self.hero_info.position.y + dash_info.direction.y * 150.0 * percent_completed;
            ctx.set_stroke_style_str("black");
            ctx.begin_path();
            let _ = ctx.arc(x as f64, y as f64, 20.0, 0.0, 2.0 * std::f64::consts::PI);
            ctx.stroke();
            return;
        }
        let x = self.hero_info.position.x as f64;
        let y = self.hero_info.position.y as f64;

        ctx.set_stroke_style_str("black");
        ctx.begin_path();
        let _ = ctx.arc(x, y, 20.0, 0.0, 2.0 * std::f64::consts::PI);
        ctx.stroke();
    }
    fn draw_direction(&self, ctx: &CanvasRenderingContext2d) {
        let mut direction = self.hero_info.direction;
        if direction.x.abs() < 0.000_001 && direction.y.abs() < 0.000_001 {
            // no direction, x & y are 0
        } else {
            direction.normalize_mut();
        }
        let x = self.hero_info.position.x + direction.x * 10.0;
        let y = self.hero_info.position.y + direction.y * 10.0;
        ctx.set_fill_style_str("green");
        ctx.begin_path();
        let _ = ctx.arc(x as f64, y as f64, 5.0, 0.0, 2.0 * std::f64::consts::PI);
        ctx.fill();
    }
    fn draw_health_bar(&self, ctx: &CanvasRenderingContext2d) {
        let start_x = 10.0;
        let start_y = 10.0;
        let bar_width = 200.0;
        let bar_height = 20.0;

        // draw red background
        ctx.set_fill_style_str("red");
        ctx.fill_rect(start_x, start_y, bar_width, bar_height);

        // draw hp left as green
        ctx.set_fill_style_str("green");
        let width = bar_width * self.hero_info.hp_left_percent() as f64;
        ctx.fill_rect(start_x, start_y, width, bar_height);
    }
    fn draw_attack(&self, ctx: &CanvasRenderingContext2d) {
        if let Some(attack_info) = &self.hero_info.attacking {
            let attack_view = AttackView::new(attack_info);
            attack_view.draw(ctx);
        }
    }
}
