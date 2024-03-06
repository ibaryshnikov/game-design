use nalgebra::{Point2, Vector2};

pub fn normalize(point: Point2<f32>) -> Point2<f32> {
    let length = (point.x * point.x + point.y * point.y).sqrt();
    // skip 0 to avoid NaN's
    if length < 0.000_001 {
        return point;
    }
    Point2::new(point.x / length, point.y / length)
}

pub fn direction_from(a: &Point2<f32>, b: &Point2<f32>) -> Vector2<f32> {
    Vector2::new(a.x - b.x, a.y - b.y)
}

pub fn distance_between(a: &Point2<f32>, b: &Point2<f32>) -> f32 {
    let diff_x = a.x - b.x;
    let diff_y = a.y - b.y;
    (diff_x * diff_x + diff_y * diff_y).sqrt()
}

// #[derive(Debug, Clone)]
// pub struct Position {
//     pub x: f32,
//     pub y: f32,
// }

// impl Position {
//     pub fn distance_to(&self, other: &Position) -> f32 {
//         let diff_x = self.x - other.x;
//         let diff_y = self.y - other.y;
//         (diff_x * diff_x + diff_y * diff_y).sqrt()
//     }
// }
