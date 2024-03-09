use nalgebra::Point2;

pub mod attack;
pub mod character;
pub mod hero;
pub mod position;
pub mod server;
pub mod types;

use attack::AttackInfo;

pub fn check_hit(
    attack_info: &AttackInfo,
    attack_distance: f32,
    target_position: Point2<f32>,
) -> bool {
    let angle = attack_info.get_base_angle();

    let width_radian = attack_info.width_radian();
    let (start_angle, end_angle) = attack_info.get_angles(angle, width_radian);

    let point_a = attack_info.position;
    let point_b = Point2::new(
        point_a.x + attack_distance * start_angle.cos(),
        point_a.y + attack_distance * start_angle.sin(),
    );
    let point_c = Point2::new(
        point_a.x + attack_distance * end_angle.cos(),
        point_a.y + attack_distance * end_angle.sin(),
    );

    let radius = 20.0;
    // hero position at the moment of attack completion
    check_points_with_circle(point_a, point_b, point_c, target_position, radius)
}

fn check_points_with_circle(
    point_a: Point2<f32>,
    point_b: Point2<f32>,
    point_c: Point2<f32>,
    center: Point2<f32>,
    radius: f32,
) -> bool {
    let line_ab = line_from_points(point_a, point_b);
    if circle_intersects_line(line_ab, center, radius) {
        return true;
    }
    let line_ac = line_from_points(point_a, point_c);
    if circle_intersects_line(line_ac, center, radius) {
        return true;
    }
    let line_bc = line_from_points(point_b, point_c);
    circle_intersects_line(line_bc, center, radius)
}

fn circle_intersects_line((a, b, c): (f32, f32, f32), center: Point2<f32>, radius: f32) -> bool {
    let distance = distance_from_line_to_point(a, b, c, center.x, center.y);
    distance < radius
}

fn distance_from_line_to_point(a: f32, b: f32, c: f32, x: f32, y: f32) -> f32 {
    (a * x + b * y + c).abs() / (a * a + b * b).sqrt()
}

fn line_from_points(point_1: Point2<f32>, point_2: Point2<f32>) -> (f32, f32, f32) {
    find_line(point_1.x, point_1.y, point_2.x, point_2.y)
}

// We need the following equation: ax + by + c = 0
// (y - y1) / (y2 - y1) = (x - x1) / (x2 - x1)
// (y - y1) * (x2 - x1) = (x - x1) * (y2 - y1)
// (x2 - x1) * y - (x2 - x1) * y1 = (y2 - y1) * x - (y2 - y1) * x1
// (y2 - y1) * x - (x2 - x1) * y - (y2 - y1) * x1 + (x2 - x1) * y1 = 0
// a = y2 - y1
// b = x1 - x2
// c = (x2 - x1) * y1 - (y2 - y1) * x1
// c = x2 * y1 - y2 * x1
fn find_line(x1: f32, y1: f32, x2: f32, y2: f32) -> (f32, f32, f32) {
    let a = y2 - y1;
    let b = x1 - x2;
    let c = x2 * y1 - y2 * x1;
    (a, b, c)
}

// some tests for lines and distances

// Checking the triangle with A: Point { x: 512.0, y: 384.0 }, B: Point { x: 362.60272, y: 370.567 }, C: Point { x: 379.62704, y: 313.4493 }
// Circle is: Point { x: 402.8994, y: 376.09946 } 20

#[test]
fn circle_intersects_one_side_of_a_triangle() {
    let point_a = Point2::new(512.0, 384.0);
    let point_b = Point2::new(362.60272, 370.567);
    let point_c = Point2::new(379.62704, 313.4493);
    let center = Point2::new(402.8994, 376.09946);
    let radius = 20.0;
    let line_ab = line_from_points(point_a, point_b);
    let (a, b, c) = line_ab;
    let sum = point_a.x * a + point_a.y * b + c;
    assert!(sum < 0.1, "Check line equation, sum should be around 0");
    let intersection = circle_intersects_line(line_ab, center, radius);
    assert!(intersection, "Circle intersects the line");
    let result = check_points_with_circle(point_a, point_b, point_c, center, radius);
    assert!(result, "Circle intersects or inside the triangle");
}

#[test]
fn circle_doesnt_intersect_the_triangle() {
    let point_a = Point2::new(512.0, 384.0);
    let point_b = Point2::new(362.60272, 370.567);
    let point_c = Point2::new(379.62704, 313.4493);
    let center = Point2::new(500.0, 500.0);
    let radius = 20.0;
    let result = check_points_with_circle(point_a, point_b, point_c, center, radius);
    assert_eq!(result, false, "Circle doesn't the triangle");
}
