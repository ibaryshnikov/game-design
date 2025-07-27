use nalgebra::Point2;

pub mod action;
pub mod attack;
pub mod character;
pub mod effect;
pub mod hero;
pub mod level;
pub mod list;
pub mod npc;
pub mod position;
pub mod projectile;
pub mod resource;

use attack::{AttackInfo, AttackKind};

pub fn check_hit(
    attack_info: &AttackInfo,
    attack_distance: f32,
    target_position: Point2<f32>,
    target_radius: f32,
) -> bool {
    use AttackKind::*;
    match attack_info.kind {
        Pizza => check_hit_arc(attack_info, attack_distance, target_position, target_radius),
        Circle => check_hit_circle(attack_info, attack_distance, target_position, target_radius),
    }
}

pub fn check_hit_circle(
    attack_info: &AttackInfo,
    attack_distance: f32,
    target_position: Point2<f32>,
    target_radius: f32,
) -> bool {
    let dx = attack_info.position.x - target_position.x;
    let dy = attack_info.position.y - target_position.y;
    let dd = (dx * dx + dy * dy).sqrt();
    dd < attack_distance + target_radius
}

pub fn check_hit_arc(
    attack_info: &AttackInfo,
    attack_distance: f32,
    target_position: Point2<f32>,
    target_radius: f32,
) -> bool {
    let angle = attack_info.get_base_angle();

    // if attack_distance < 110.0 {
    //     println!("attack distance {attack_distance}, target_radius: {target_radius}");
    // }

    let dx = attack_info.position.x - target_position.x;
    let dy = attack_info.position.y - target_position.y;
    let direction_angle = dy.atan2(dx) + std::f32::consts::PI;

    let width_radian = attack_info.width_radian();
    let (start_angle, end_angle) = attack_info.get_angles(angle, width_radian);

    // if attack_distance < 110.0 {
    //     println!(
    //         "direction_angle {direction_angle}, start_angle {start_angle}, end_angle {end_angle}"
    //     );
    // }

    if direction_angle > start_angle && direction_angle < end_angle {
        let dd = (dx * dx + dy * dy).sqrt();
        // if attack_distance < 110.0 {
        //     println!("dd {dd}");
        // }
        return dd < attack_distance + target_radius;
    }

    let point_a = attack_info.position;
    let point_b = Point2::new(
        point_a.x + attack_distance * start_angle.cos(),
        point_a.y + attack_distance * start_angle.sin(),
    );
    let point_c = Point2::new(
        point_a.x + attack_distance * end_angle.cos(),
        point_a.y + attack_distance * end_angle.sin(),
    );

    // hero position at the moment of attack completion
    check_points_with_circle(point_a, point_b, point_c, target_position, target_radius)
}

fn check_points_with_circle(
    point_a: Point2<f32>,
    point_b: Point2<f32>,
    point_c: Point2<f32>,
    center: Point2<f32>,
    radius: f32,
) -> bool {
    circle_intersects_line_segment(point_a, point_b, center, radius)
        || circle_intersects_line_segment(point_a, point_c, center, radius)
}

// line equation:
// x = x1 + t*(x2 - x1), t ∈ [0, 1]
// y = y1 + t*(y2 - y1), t ∈ [0, 1]
//
// circle equation:
// (x - a)^2 + (y - b)^2 = R^2
//
// combine together:
// (t*(x2 - x1) + x1 - a)^2 + (t*(y2 - y1) + y1 - b)^2 = R^2
// let p = x2 - x1
//     q = x1 - a
//     k = y2 - y1
//     l = y1 - b
// then
// (p^2 + k^2)*t^2 + (2*p*q + 2*k*l)*t + q^2 +l^2-R^2 = 0
// let a = p^2 + k^2
//     b = 2*p*q + 2*k*l
//     c = q^2 + l^2 - R^2
// then
// a*t^2 + b*t + c = 0
fn circle_intersects_line_segment(
    start: Point2<f32>,
    end: Point2<f32>,
    center: Point2<f32>,
    radius: f32,
) -> bool {
    let p = end.x - start.x;
    let q = start.x - center.x;
    let k = end.y - start.y;
    let l = start.y - center.y;
    let a = p * p + k * k;
    let b = 2.0 * p * q + 2.0 * k * l;
    let c = q * q + l * l - radius * radius;
    let d = b * b - 4.0 * a * c;
    // no solutions
    if d < 0.0 {
        return false;
    }
    let range = 0.0..=1.0;
    // single solution
    if d == 0.0 {
        let t = -b / (2.0 * a);
        return range.contains(&t);
        // return t >= 0.0 && t <= 1.0;
    }
    // two solutions
    let t1 = (-b + d.sqrt()) / (2.0 * a);
    if range.contains(&t1) {
        return true;
    }
    let t2 = (-b - d.sqrt()) / (2.0 * a);
    range.contains(&t2)
}
