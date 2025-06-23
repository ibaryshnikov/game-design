use web_sys::CanvasRenderingContext2d;

use shared::attack::{
    AttackInfo, AttackKind, AttackOrder, AttackSequence, AttackShape, AttackState, ComplexAttack,
};

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
        // match self.attack_info.kind {
        //     AttackKind::Pizza => self.draw_selected_arc(ctx),
        //     AttackKind::Circle => self.draw_selected_circle(ctx),
        // }
    }
    // fn draw_selected_arc(&self, ctx: &CanvasRenderingContext2d) {
    //     let width_radian = self.width_radian();
    //     let radius = self.attack_info.distance;
    //     let angle = self.attack_info.get_base_angle();
    //
    //     let actor_position = self.attack_info.position;
    //     let start = iced_core::Point::new(actor_position.x, actor_position.y);
    //
    //     let start_angle = angle - width_radian;
    //     let end_angle = angle + width_radian;
    //     let path = circle_segment(start, radius, start_angle, end_angle);
    //
    //     ctx.stroke(
    //         &path,
    //         Stroke {
    //             style: stroke::Style::Solid(Color::from_rgb8(0, 255, 0)),
    //                  width: 3.0,
    //                  ..Stroke::default()
    //         },
    //     );
    // }
    // fn draw_selected_circle(&self, ctx: &CanvasRenderingContext2d) {
    //     let info = &self.attack_info;
    //     let path = Path::new(|b| {
    //         b.circle(
    //             iced_core::Point::new(info.position.x, info.position.y),
    //                  info.distance,
    //         );
    //     });
    //     ctx.stroke(
    //         &path,
    //         Stroke {
    //             style: stroke::Style::Solid(Color::from_rgb8(0, 255, 0)),
    //                  width: 3.0,
    //                  ..Stroke::default()
    //         },
    //     );
    // }
    fn draw_attacking(&self, ctx: &CanvasRenderingContext2d) {
        match self.attack_info.kind {
            AttackKind::Pizza => self.draw_attacking_arc(ctx),
            AttackKind::Circle => {
                if self.attack_info.damage_done {
                    return;
                }
                self.draw_attacking_circle(ctx);
            }
        }
    }
    fn draw_attacking_arc(&self, ctx: &CanvasRenderingContext2d) {
        // match self.attack_info.order {
        //     AttackOrder::LeftThenRight | AttackOrder::RightThenLeft => {
        //         self.draw_two_parts(ctx);
        //         return;
        //     }
        //     _ => (),
        // }
        // let info = &self.attack_info;
        // let width_radian = info.width_radian();
        // let radius = info.get_radius();
        // let angle = info.get_base_angle();
        //
        // let (start_angle, end_angle) = info.get_angles(angle, width_radian);
        // let start = iced_core::Point::new(info.position.x, info.position.y);
        // draw_circle_segment(ctx, start, radius, start_angle, end_angle);
        //
        // if let AttackOrder::SidesToCenter = info.order {
        //     let end_angle = angle + width_radian;
        //     let start_angle = end_angle - width_radian * info.percent_completed;
        //     draw_circle_segment(ctx, start, radius, start_angle, end_angle)
        // }
    }
    // fn draw_two_parts(&self, ctx: &CanvasRenderingContext2d) {
    //     let info = &self.attack_info;
    //     let width_radian = info.width_radian();
    //     let radius = info.get_radius();
    //     let angle = info.get_base_angle();
    //     let start = iced_core::Point::new(info.position.x, info.position.y);
    //
    //     if info.percent_completed < 0.5 {
    //         let start_angle = angle - width_radian;
    //         let end_angle = start_angle + 2.0 * width_radian * info.percent_completed;
    //         draw_circle_segment(ctx, start, radius, start_angle, end_angle);
    //     } else {
    //         // draw first part
    //         let start_angle = angle - width_radian;
    //         let end_angle = angle;
    //         draw_circle_segment(ctx, start, radius, start_angle, end_angle);
    //
    //         // draw second part
    //         let end_angle = angle + width_radian;
    //         let start_angle = end_angle - 2.0 * width_radian * (info.percent_completed - 0.5);
    //         draw_circle_segment(ctx, start, radius, start_angle, end_angle);
    //     }
    // }
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

// fn draw_circle_segment(
//     ctx: &CanvasRenderingContext2d,
//     center: iced_core::Point,
//     radius: f32,
//     start_angle: f32,
//     end_angle: f32,
// ) {
//     let path = circle_segment(center, radius, start_angle, end_angle);
//     ctx.fill(&path, Color::from_rgb8(255, 0, 0));
// }
//
// fn circle_segment(center: iced_core::Point, radius: f32, start_angle: f32, end_angle: f32) -> Path {
//     // println!("center.x {} center.y {}", center.x, center.y);
//     // println!("radius {} start angle {} end angle {}", radius, start_angle, end_angle);
//     let side = iced_core::Point::new(
//         center.x + radius * start_angle.cos(),
//                                      center.y + radius * start_angle.sin(),
//     );
//     Path::new(|b| {
//         b.move_to(center);
//         b.line_to(side);
//         b.arc(canvas::path::arc::Arc {
//             center,
//             radius,
//             start_angle: start_angle.into(),
//               end_angle: end_angle.into(),
//         });
//         b.line_to(center);
//     })
// }
