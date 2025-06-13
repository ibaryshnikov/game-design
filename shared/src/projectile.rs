use nalgebra::{Point2, Vector2};

pub struct Projectile {
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub speed: f32,
}

impl Projectile {
    pub fn update(&mut self, dt: u128) {
        self.position.x += self.direction.x * self.speed * dt as f32;
        self.position.y += self.direction.y * self.speed * dt as f32;
    }
}
