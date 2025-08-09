use web_sys::CanvasRenderingContext2d;

use shared::attack::{AttackInfo, AttackKind, AttackOrder, AttackState};
// use shared::attack::{AttackSequence, AttackShape, ComplexAttack};

// drawing a sword
// const points = [
//     [-15, -20],
//     [-5, -20],
//     [-5, -70],
//     [0, -75],
//     [5, -70],
//     [5, -20],
//     [15, -20],
//     [15, -15],
//     [3, -15],
//     [3, 0],
//     [-3, 0],
//     [-3, -15],
//     [-15, -15],
//     [-15, -20],
// ];
// drawAt({ x: 130, y: 120 }, points);
// function drawAt(position, points) {
//     ctx.rotate(0.1 * Math.PI);
//     ctx.strokeStyle = 'red';
//     ctx.beginPath();
//     for ([x, y] of points) {
//         ctx.lineTo(position.x + x, position.y + y);
//     }
//     ctx.closePath();
//     ctx.stroke();
// }

// pub struct ComplexAttackView<'a> {
//     pub complex_attack: &'a ComplexAttack,
// }
//
// impl<'a> ComplexAttackView<'a> {
//     pub fn new(complex_attack: &'a ComplexAttack) -> Self {
//         Self { complex_attack }
//     }
//     pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
//         for sequence in self.complex_attack.sequences.iter() {
//             draw_sequence(ctx, sequence);
//         }
//     }
// }
//
// fn draw_sequence(ctx: &CanvasRenderingContext2d, sequence: &AttackSequence) {
//     let Some(part) = &sequence.active_part() else {
//         return;
//     };
//     match &part.shape {
//         AttackShape::Circle(circle) => {
//             let path = Path::new(|b| {
//                 b.circle(
//                     iced_core::Point::new(circle.position.x, circle.position.y),
//                          circle.radius,
//                 );
//             });
//             ctx.fill(&path, Color::from_rgb8(255, 0, 0));
//         }
//         AttackShape::Pizza(pizza) => {
//             let width_radian = pizza.width_radian();
//             let radius = pizza.get_radius();
//             let angle = pizza.get_base_angle();
//
//             let (start_angle, end_angle) = pizza.get_angles(angle, width_radian);
//             let start = iced_core::Point::new(pizza.position.x, pizza.position.y);
//             draw_circle_segment(ctx, start, radius, start_angle, end_angle);
//
//             if let AttackOrder::SidesToCenter = pizza.order {
//                 let end_angle = angle + width_radian;
//                 let start_angle = end_angle - width_radian * pizza.percent_completed;
//                 draw_circle_segment(ctx, start, radius, start_angle, end_angle)
//             }
//         }
//         _ => (),
//     }
// }

pub struct AttackView<'a> {
    pub attack_info: &'a AttackInfo,
}

impl<'a> AttackView<'a> {
    pub fn new(attack_info: &'a AttackInfo) -> Self {
        Self { attack_info }
    }
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        match self.attack_info.state {
            AttackState::Selected => self.draw_selected(ctx),
            AttackState::Attacking => self.draw_attacking(ctx),
        }
    }
    pub fn width_radian(&self) -> f32 {
        self.attack_info.width_radian()
    }
    fn draw_selected(&self, ctx: &CanvasRenderingContext2d) {
        match self.attack_info.kind {
            AttackKind::Pizza => self.draw_selected_arc(ctx),
            AttackKind::Circle => self.draw_selected_circle(ctx),
        }
    }
    fn draw_selected_arc(&self, ctx: &CanvasRenderingContext2d) {
        let width_radian = self.width_radian();
        let radius = self.attack_info.distance as f64;
        let angle = self.attack_info.get_base_angle();

        let actor_position = self.attack_info.position;
        let start_x = actor_position.x as f64;
        let start_y = actor_position.y as f64;

        let start_angle = angle - width_radian;
        let end_angle = angle + width_radian;

        ctx.set_stroke_style_str("green");
        ctx.begin_path();
        circle_segment(
            ctx,
            start_x,
            start_y,
            radius,
            start_angle as f64,
            end_angle as f64,
        );
        ctx.stroke();
    }
    fn draw_selected_circle(&self, ctx: &CanvasRenderingContext2d) {
        let info = &self.attack_info;
        let x = info.position.x as f64;
        let y = info.position.y as f64;
        let radius = info.distance as f64;

        ctx.set_fill_style_str("green");
        ctx.begin_path();
        let _ = ctx.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
        ctx.stroke();
    }
    fn draw_attacking(&self, ctx: &CanvasRenderingContext2d) {
        match self.attack_info.kind {
            AttackKind::Pizza => self.draw_attacking_arc(ctx),
            AttackKind::Circle => {
                self.draw_attacking_circle(ctx);
            }
        }
    }
    fn draw_attacking_arc(&self, ctx: &CanvasRenderingContext2d) {
        match self.attack_info.order {
            AttackOrder::LeftThenRight | AttackOrder::RightThenLeft => {
                self.draw_two_parts(ctx);
                return;
            }
            _ => (),
        }
        let info = &self.attack_info;
        let width_radian = info.width_radian();
        let radius = info.get_radius() as f64;
        let angle = info.get_base_angle();

        let (start_angle, end_angle) = info.get_angles(angle, width_radian);

        // console_log!("start_angle {start_angle}, end_angle {end_angle}, width_radian {width_radian}");

        let start_x = info.position.x as f64;
        let start_y = info.position.y as f64;
        draw_circle_segment(
            ctx,
            start_x,
            start_y,
            radius,
            start_angle as f64,
            end_angle as f64,
        );

        if let AttackOrder::SidesToCenter = info.order {
            let end_angle = angle + width_radian;
            let start_angle = end_angle - width_radian * info.percent_completed;
            draw_circle_segment(
                ctx,
                start_x,
                start_y,
                radius,
                start_angle as f64,
                end_angle as f64,
            );
        }
    }
    fn draw_two_parts(&self, ctx: &CanvasRenderingContext2d) {
        let info = &self.attack_info;
        let width_radian = info.width_radian();
        let radius = info.get_radius() as f64;
        let angle = info.get_base_angle();
        let start_x = info.position.x as f64;
        let start_y = info.position.y as f64;

        if info.percent_completed < 0.5 {
            let start_angle = angle - width_radian;
            let end_angle = start_angle + 2.0 * width_radian * info.percent_completed;
            draw_circle_segment(
                ctx,
                start_x,
                start_y,
                radius,
                start_angle as f64,
                end_angle as f64,
            );
        } else {
            // draw first part
            let start_angle = angle - width_radian;
            let end_angle = angle;
            draw_circle_segment(
                ctx,
                start_x,
                start_y,
                radius,
                start_angle as f64,
                end_angle as f64,
            );

            // draw second part
            let end_angle = angle + width_radian;
            let start_angle = end_angle - 2.0 * width_radian * (info.percent_completed - 0.5);
            draw_circle_segment(
                ctx,
                start_x,
                start_y,
                radius,
                start_angle as f64,
                end_angle as f64,
            );
        }
    }
    fn draw_attacking_circle(&self, ctx: &CanvasRenderingContext2d) {
        let info = &self.attack_info;
        let x = info.position.x as f64;
        let y = info.position.y as f64;
        let radius = info.get_radius() as f64;

        ctx.set_fill_style_str("red");
        ctx.begin_path();
        let _ = ctx.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
        ctx.fill();
    }
}

fn draw_circle_segment(
    ctx: &CanvasRenderingContext2d,
    center_x: f64,
    center_y: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) {
    ctx.set_fill_style_str("red");
    ctx.begin_path();
    circle_segment(ctx, center_x, center_y, radius, start_angle, end_angle);
    ctx.fill();
}

fn circle_segment(
    ctx: &CanvasRenderingContext2d,
    center_x: f64,
    center_y: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) {
    // console_log!("center.x {} center.y {}", center.x, center.y);
    // console_log!("radius {} start angle {} end angle {}", radius, start_angle, end_angle);
    let side_x = center_x + radius * start_angle.cos();
    let side_y = center_y + radius * start_angle.sin();
    ctx.move_to(center_x, center_y);
    ctx.line_to(side_x, side_y);
    let counterclockwise = start_angle > end_angle;
    let _ = ctx.arc_with_anticlockwise(
        center_x,
        center_y,
        radius,
        start_angle,
        end_angle,
        counterclockwise,
    );
    ctx.line_to(center_x, center_y);
}
