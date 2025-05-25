use iced_core::Color;
use iced_widget::canvas::{self, stroke, Frame, Path, Stroke};

use shared::attack::{AttackInfo, AttackKind, AttackOrder, AttackState};

pub struct AttackView<'a> {
    pub attack_info: &'a AttackInfo,
}

impl<'a> AttackView<'a> {
    pub fn new(attack_info: &'a AttackInfo) -> Self {
        Self { attack_info }
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
        match self.attack_info.kind {
            AttackKind::Pizza => self.draw_selected_arc(frame),
            AttackKind::Circle => self.draw_selected_circle(frame),
        }
    }
    fn draw_selected_arc(&self, frame: &mut Frame) {
        let width_radian = self.width_radian();
        let radius = self.attack_info.distance;
        let angle = self.attack_info.get_base_angle();

        let actor_position = self.attack_info.position;
        let start = iced_core::Point::new(actor_position.x, actor_position.y);

        let start_angle = angle - width_radian;
        let end_angle = angle + width_radian;
        let path = circle_segment(start, radius, start_angle, end_angle);

        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(Color::from_rgb8(0, 255, 0)),
                width: 3.0,
                ..Stroke::default()
            },
        );
    }
    fn draw_selected_circle(&self, frame: &mut Frame) {
        let info = &self.attack_info;
        let path = Path::new(|b| {
            b.circle(
                iced_core::Point::new(info.position.x, info.position.y),
                info.distance,
            );
        });
        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(Color::from_rgb8(0, 255, 0)),
                width: 3.0,
                ..Stroke::default()
            },
        );
    }
    fn draw_attacking(&self, frame: &mut Frame) {
        match self.attack_info.kind {
            AttackKind::Pizza => self.draw_attacking_arc(frame),
            AttackKind::Circle => {
                if self.attack_info.damage_done {
                    return;
                }
                self.draw_attacking_circle(frame);
            }
        }
    }
    fn draw_attacking_arc(&self, frame: &mut Frame) {
        match self.attack_info.order {
            AttackOrder::LeftThenRight | AttackOrder::RightThenLeft => {
                self.draw_two_parts(frame);
                return;
            }
            _ => (),
        }
        let info = &self.attack_info;
        let width_radian = info.width_radian();
        let radius = info.get_radius();
        let angle = info.get_base_angle();

        let (start_angle, end_angle) = info.get_angles(angle, width_radian);
        let start = iced_core::Point::new(info.position.x, info.position.y);
        draw_circle_segment(frame, start, radius, start_angle, end_angle);

        if let AttackOrder::SidesToCenter = info.order {
            let end_angle = angle + width_radian;
            let start_angle = end_angle - width_radian * info.percent_completed;
            draw_circle_segment(frame, start, radius, start_angle, end_angle)
        }
    }
    fn draw_two_parts(&self, frame: &mut Frame) {
        let info = &self.attack_info;
        let width_radian = info.width_radian();
        let radius = info.get_radius();
        let angle = info.get_base_angle();
        let start = iced_core::Point::new(info.position.x, info.position.y);

        if info.percent_completed < 0.5 {
            let start_angle = angle - width_radian;
            let end_angle = start_angle + 2.0 * width_radian * info.percent_completed;
            draw_circle_segment(frame, start, radius, start_angle, end_angle);
        } else {
            // draw first part
            let start_angle = angle - width_radian;
            let end_angle = angle;
            draw_circle_segment(frame, start, radius, start_angle, end_angle);

            // draw second part
            let end_angle = angle + width_radian;
            let start_angle = end_angle - 2.0 * width_radian * (info.percent_completed - 0.5);
            draw_circle_segment(frame, start, radius, start_angle, end_angle);
        }
    }
    fn draw_attacking_circle(&self, frame: &mut Frame) {
        let info = &self.attack_info;
        let radius = info.get_radius();
        let path = Path::new(|b| {
            b.circle(
                iced_core::Point::new(info.position.x, info.position.y),
                radius,
            );
        });
        frame.fill(&path, Color::from_rgb8(255, 0, 0));
    }
}

fn draw_circle_segment(
    frame: &mut Frame,
    center: iced_core::Point,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
) {
    let path = circle_segment(center, radius, start_angle, end_angle);
    frame.fill(&path, Color::from_rgb8(255, 0, 0));
}

fn circle_segment(center: iced_core::Point, radius: f32, start_angle: f32, end_angle: f32) -> Path {
    // println!("center.x {} center.y {}", center.x, center.y);
    // println!("radius {} start angle {} end angle {}", radius, start_angle, end_angle);
    let side = iced_core::Point::new(
        center.x + radius * start_angle.cos(),
        center.y + radius * start_angle.sin(),
    );
    Path::new(|b| {
        b.move_to(center);
        b.line_to(side);
        b.arc(canvas::path::arc::Arc {
            center,
            radius,
            start_angle: start_angle.into(),
            end_angle: end_angle.into(),
        });
        b.line_to(center);
    })
}
