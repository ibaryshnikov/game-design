use iced_core::{Color, Size};
use iced_widget::canvas::{stroke, Frame, Path, Stroke};

use game_core::hero::Hero;

use crate::attack::AttackView;

pub struct HeroView {
    pub hero_info: Hero,
}

impl HeroView {
    pub fn draw_body(&self, frame: &mut Frame) {
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
        self.draw_direction(frame);
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
    pub fn draw_health_bar(&self, frame: &mut Frame) {
        // self.draw_test_data(frame);
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
    fn draw_test_data(&self, frame: &mut Frame) {
        let point_a = iced_core::Point::new(512.0, 384.0);
        let point_b = iced_core::Point::new(362.60272, 370.567);
        let point_c = iced_core::Point::new(379.62704, 313.4493);
        let center = iced_core::Point::new(402.8994, 376.09946);
        let radius = 20.0;
        let path = Path::new(|b| {
            b.move_to(point_a);
            b.line_to(point_b);
            b.line_to(point_c);
            b.line_to(point_a);
            b.move_to(center);
            b.circle(center, radius);
        });
        frame.fill(&path, Color::from_rgb8(0, 0, 255));
    }
    pub fn draw_attack(&self, frame: &mut Frame) {
        if let Some(attack_info) = &self.hero_info.attacking {
            let attack_view = AttackView::new(attack_info);
            attack_view.draw(frame);
        }
    }
}
