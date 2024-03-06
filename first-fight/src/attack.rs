use iced::widget::canvas::{self, stroke, Frame, Path, Stroke};
use iced::Color;

use shared::attack::{AttackInfo, AttackOrder, AttackState};

pub struct AttackView {
    pub attack_info: AttackInfo,
}

impl AttackView {
    pub fn new(attack_info: AttackInfo) -> Self {
        Self { attack_info }
    }

    pub fn update(&mut self) {
        self.attack_info.update();
    }
    pub fn completed(&self) -> bool {
        self.attack_info.completed()
    }
    pub fn draw(&self, frame: &mut Frame) {
        match self.attack_info.state {
            AttackState::Selected => self.draw_selected(frame),
            AttackState::Attacking => self.draw_attacking(frame),
        }
    }
    pub fn width_radian(&self) -> f32 {
        self.attack_info.width_radian()
    }
    fn draw_selected(&self, frame: &mut Frame) {
        let width_radian = self.width_radian();
        let radius = self.attack_info.distance;
        let angle = self.attack_info.get_base_angle();

        let actor_position = self.attack_info.position;
        let start = iced::Point::new(actor_position.x, actor_position.y);

        let start_angle = angle - width_radian;
        let end_angle = angle + width_radian;
        let path = circle_segment(start, radius, start_angle, end_angle);

        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(Color::new(0.0, 1.0, 0.0, 1.0)),
                width: 3.0,
                ..Stroke::default()
            },
        );
    }
    fn draw_left_then_right(&self, frame: &mut Frame) {
        let width_radian = self.attack_info.width_radian();
        let radius = self.attack_info.get_radius();
        let angle = self.attack_info.get_base_angle();
        let actor_position =
            iced::Point::new(self.attack_info.position.x, self.attack_info.position.y);

        if self.attack_info.percent_completed < 0.5 {
            let start_angle = angle - width_radian;
            let end_angle = start_angle + 2.0 * width_radian * self.attack_info.percent_completed;

            let start = actor_position;

            let path = circle_segment(start, radius, start_angle, end_angle);

            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));
        } else {
            // draw first part
            let start_angle = angle - width_radian;
            let end_angle = angle;

            let start = actor_position;

            let path = circle_segment(start, radius, start_angle, end_angle);

            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));

            // draw second part
            let end_angle = angle + width_radian;
            let start_angle =
                end_angle - 2.0 * width_radian * (self.attack_info.percent_completed - 0.5);

            let start = actor_position;

            let path = circle_segment(start, radius, start_angle, end_angle);

            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));
        }
    }
    fn draw_right_then_left(&self, frame: &mut Frame) {
        let width_radian = self.attack_info.width_radian();
        let radius = self.attack_info.get_radius();
        let angle = self.attack_info.get_base_angle();
        let actor_position =
            iced::Point::new(self.attack_info.position.x, self.attack_info.position.y);

        if self.attack_info.percent_completed < 0.5 {
            let end_angle = angle + width_radian;
            let start_angle = end_angle - 2.0 * width_radian * self.attack_info.percent_completed;

            let start = actor_position;

            let path = circle_segment(start, radius, start_angle, end_angle);

            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));
        } else {
            // draw first part
            let end_angle = angle + width_radian;
            let start_angle = angle;

            let start = actor_position;

            let path = circle_segment(start, radius, start_angle, end_angle);

            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));

            // draw second part
            let start_angle = angle - width_radian;
            let end_angle =
                start_angle + 2.0 * width_radian * (self.attack_info.percent_completed - 0.5);

            let path = circle_segment(start, radius, start_angle, end_angle);

            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));
        }
    }
    fn draw_attacking(&self, frame: &mut Frame) {
        match self.attack_info.order {
            AttackOrder::LeftThenRight => {
                self.draw_left_then_right(frame);
                return;
            }
            AttackOrder::RightThenLeft => {
                self.draw_right_then_left(frame);
                return;
            }
            _ => (),
        }
        let width_radian = self.attack_info.width_radian();
        let radius = self.attack_info.get_radius();
        let angle = self.attack_info.get_base_angle();

        let (start_angle, end_angle) = self.attack_info.get_angles(angle, width_radian);

        let start = iced::Point::new(self.attack_info.position.x, self.attack_info.position.y);

        let path = circle_segment(start, radius, start_angle, end_angle);

        frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));

        if let AttackOrder::SidesToCenter = self.attack_info.order {
            let end_angle = angle + width_radian;
            let start_angle = end_angle - width_radian * self.attack_info.percent_completed;
            let path = circle_segment(start, radius, start_angle, end_angle);
            frame.fill(&path, Color::new(1.0, 0.0, 0.0, 1.0));
        }
    }
}

fn circle_segment(center: iced::Point, radius: f32, start_angle: f32, end_angle: f32) -> Path {
    let side = iced::Point::new(
        center.x + radius * start_angle.cos(),
        center.y + radius * start_angle.sin(),
    );
    Path::new(|b| {
        b.move_to(center);
        b.line_to(side);
        b.arc(canvas::path::arc::Arc {
            center,
            radius,
            start_angle: iced::Radians(start_angle),
            end_angle: iced::Radians(end_angle),
        });
        b.line_to(center);
    })
}
