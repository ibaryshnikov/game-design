use iced_core::{Color, Size};
use iced_widget::canvas::{Frame, Path, Stroke, stroke};

use game_core::hero::Hero;

use crate::attack::AttackView;

pub struct HeroView<'a> {
    pub hero_info: &'a Hero,
}

impl<'a> HeroView<'a> {
    pub fn new(hero_info: &'a Hero) -> Self {
        Self { hero_info }
    }
    pub fn draw(&self, frame: &mut Frame) {
        self.draw_body(frame);
        self.draw_direction(frame);
        self.draw_attack(frame);
        self.draw_health_bar(frame);
    }
    fn draw_body(&self, frame: &mut Frame) {
        if let Some(dash_info) = &self.hero_info.dashing {
            let percent_completed = dash_info.percent_completed();
            let position = iced_core::Point::new(
                self.hero_info.position.x + dash_info.direction.x * 150.0 * percent_completed,
                self.hero_info.position.y + dash_info.direction.y * 150.0 * percent_completed,
            );
            let path = Path::new(|b| {
                b.circle(position, 20.0);
            });
            frame.stroke(
                &path,
                Stroke {
                    style: stroke::Style::Solid(Color::BLACK),
                    width: 3.0,
                    ..Stroke::default()
                },
            );
            return;
        }
        let path = Path::new(|b| {
            b.circle(
                iced_core::Point::new(self.hero_info.position.x, self.hero_info.position.y),
                20.0,
            );
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
    fn draw_direction(&self, frame: &mut Frame) {
        let mut direction = self.hero_info.direction;
        if direction.x.abs() < 0.000_001 && direction.y.abs() < 0.000_001 {
            // no direction, x & y are 0
        } else {
            direction.normalize_mut();
        }
        let start = iced_core::Point::new(
            self.hero_info.position.x + direction.x * 10.0,
            self.hero_info.position.y + direction.y * 10.0,
        );
        let path = Path::new(|b| {
            b.circle(start, 5.0);
        });

        frame.fill(&path, Color::from_rgb8(0, 255, 0));
    }
    fn draw_health_bar(&self, frame: &mut Frame) {
        let start = iced_core::Point::new(10.0, 10.0);
        let bar_width = 200.0;
        let bar_height = 20.0;

        // draw red background
        let path = Path::new(|b| {
            let size = Size::new(bar_width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(255, 0, 0));

        // draw hp left as green
        let path = Path::new(|b| {
            let width = bar_width * self.hero_info.hp_left_percent();
            let size = Size::new(width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(0, 255, 0));
    }
    fn draw_attack(&self, frame: &mut Frame) {
        if let Some(attack_info) = &self.hero_info.attacking {
            let attack_view = AttackView::new(attack_info);
            attack_view.draw(frame);
        }
    }
}
