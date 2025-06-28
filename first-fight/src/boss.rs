use iced_core::{Color, Size};
use iced_widget::canvas::{Frame, Path, Stroke, stroke};

use game_core::boss::Boss;

use crate::attack::{AttackView, ComplexAttackView};

pub struct BossView<'a> {
    pub boss_info: &'a Boss,
}

impl<'a> BossView<'a> {
    pub fn new(boss_info: &'a Boss) -> Self {
        Self { boss_info }
    }
    pub fn draw(&self, frame: &mut Frame) {
        self.draw_body(frame);
        self.draw_attack(frame);
        self.draw_health_bar(frame);
    }
    pub fn draw_body(&self, frame: &mut Frame) {
        let position = iced_core::Point::new(self.boss_info.position.x, self.boss_info.position.y);
        let path = Path::new(|b| {
            b.circle(position, 30.0);
        });
        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(Color::BLACK),
                width: 3.0,
                ..Stroke::default()
            },
        );
    }
    pub fn draw_health_bar(&self, frame: &mut Frame) {
        let start = iced_core::Point::new(100.0, 700.0);
        let bar_width = 800.0;
        let bar_height = 10.0;

        // draw black background
        let path = Path::new(|b| {
            let size = Size::new(bar_width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(0, 0, 0));

        // draw hp left as green
        let path = Path::new(|b| {
            let width = bar_width * self.boss_info.hp_left_percent();
            let size = Size::new(width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(255, 0, 0));
    }
    pub fn draw_attack(&self, frame: &mut Frame) {
        if let Some(attack_info) = &self.boss_info.attacking {
            let attack_view = AttackView::new(attack_info);
            attack_view.draw(frame);
        }
        if let Some(attack) = &self.boss_info.attacking_complex {
            let attack_view = ComplexAttackView::new(attack);
            attack_view.draw(frame);
        }
    }
}
