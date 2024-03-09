use std::time::Instant;

use nalgebra::{Point2, Vector2};

#[derive(Debug)]
pub struct SelectionInfo {
    pub position: Point2<f32>,
    pub selected_at: Instant,
}

#[derive(Debug)]
pub enum MissileShape {
    Circle,
}

#[derive(Debug)]
pub struct MissileInfo {
    pub number_of_missiles: u8,
    pub shape: MissileShape,
}

#[derive(Debug)]
pub enum AttackKind {
    Narrow,
    Wide,
    CustomAngle(f32),
    Circle,
    Missiles(Box<MissileInfo>),
}

#[derive(Debug)]
pub enum AttackOrder {
    LeftToRight,
    RightToLeft,
    LeftThenRight,
    RightThenLeft,
    CloseToFar,
    CenterToSides,
    SidesToCenter,
    ExpandingCircle,
    ProjectileFromCaster,
}

#[derive(Debug)]
pub enum AttackState {
    Selected,
    Attacking,
}

#[derive(Debug)]
pub struct AttackInfo {
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub started_at: Instant,
    pub time_passed: u128,
    pub delay: u128,
    pub time_to_complete: u128, // ms
    pub aftercast: u128,
    pub percent_completed: f32,
    pub kind: AttackKind,
    pub order: AttackOrder,
    pub distance: f32,
    pub state: AttackState,
}

impl AttackInfo {
    fn new(
        position: Point2<f32>,
        direction: Vector2<f32>,
        distance: f32,
        kind: AttackKind,
        order: AttackOrder,
    ) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 700,
            time_to_complete: 600,
            aftercast: 600,
            percent_completed: 0.0,
            kind,
            order,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn narrow(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        // AttackInfo::new(position, distance, AttackKind::Narrow, AttackOrder::CloseToFar)
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 100,
            aftercast: 500,
            percent_completed: 0.0,
            kind: AttackKind::Narrow,
            order: AttackOrder::CloseToFar,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn wide(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Wide,
            order: AttackOrder::LeftToRight,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn wide_right(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Wide,
            order: AttackOrder::RightToLeft,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn left_then_right(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Wide,
            order: AttackOrder::LeftThenRight,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn right_then_left(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Wide,
            order: AttackOrder::RightThenLeft,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn split(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        // AttackInfo::new(position, distance, AttackKind::Wide, AttackOrder::CenterToSides)
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Wide,
            order: AttackOrder::CenterToSides,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn closing(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        // AttackInfo::new(position, distance, AttackKind::Wide, AttackOrder::SidesToCenter)
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Wide,
            order: AttackOrder::SidesToCenter,
            distance,
            state: AttackState::Selected,
        }
    }
    pub fn fireball(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo::new(
            position,
            direction,
            distance,
            AttackKind::Circle,
            AttackOrder::ExpandingCircle,
        )
    }
    pub fn magic_missiles(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo::new(
            position,
            direction,
            distance,
            AttackKind::Circle,
            AttackOrder::ExpandingCircle,
        )
    }
    // pub fn random_close_range(position: Point, distance: f32) -> Self {
    //     let time = Instant::epo
    // }
    pub fn random_melee(_position: Point2<f32>) {}
    pub fn random_ranged(_position: Point2<f32>) {}
}

impl AttackInfo {
    pub fn update(&mut self) {
        match self.state {
            AttackState::Selected => {
                if self.started_at.elapsed().as_millis() < self.delay {
                    return;
                }
                self.state = AttackState::Attacking;
                self.started_at = Instant::now();
            }
            AttackState::Attacking => {
                let time_passed = self.started_at.elapsed().as_millis();
                self.time_passed = time_passed;
                let mut percent_completed = time_passed as f32 / self.time_to_complete as f32;
                if percent_completed > 1.0 {
                    percent_completed = 1.0;
                }
                self.percent_completed = percent_completed;
            }
        }
    }
    pub fn completed(&self) -> bool {
        self.time_passed > self.time_to_complete + self.aftercast
    }
    pub fn width_radian(&self) -> f32 {
        let radian = match self.kind {
            AttackKind::Narrow => 0.2,
            AttackKind::Wide => 1.7,
            AttackKind::CustomAngle(angle) => angle,
            _ => 0.2,
        };
        if let AttackOrder::RightToLeft | AttackOrder::RightThenLeft = self.order {
            -radian
        } else {
            radian
        }
    }
    pub fn get_base_angle(&self) -> f32 {
        let tan_a = self.direction.y / self.direction.x;
        let mut angle = tan_a.atan();
        if self.direction.x > 0.0 {
            angle += std::f32::consts::PI;
        }
        angle
    }
    pub fn get_angles(&self, angle: f32, width_radian: f32) -> (f32, f32) {
        match self.order {
            AttackOrder::CloseToFar => (angle - width_radian, angle + width_radian),
            AttackOrder::LeftToRight | AttackOrder::RightToLeft => {
                let start_angle = angle - width_radian;
                let end_angle = start_angle + 2.0 * width_radian * self.percent_completed;
                (start_angle, end_angle)
            }
            AttackOrder::CenterToSides => {
                let width = width_radian * self.percent_completed;
                (angle - width, angle + width)
            }
            AttackOrder::SidesToCenter => {
                let start_angle = angle - width_radian;
                let end_angle = start_angle + width_radian * self.percent_completed;
                (start_angle, end_angle)
            }
            _ => (angle - width_radian, angle + width_radian),
        }
    }
    pub fn get_radius(&self) -> f32 {
        match self.order {
            AttackOrder::CloseToFar => self.distance * self.percent_completed,
            AttackOrder::LeftToRight => self.distance,
            AttackOrder::RightToLeft => self.distance,
            AttackOrder::SidesToCenter => self.distance,
            AttackOrder::CenterToSides => self.distance,
            AttackOrder::LeftThenRight => self.distance,
            AttackOrder::RightThenLeft => self.distance,
            AttackOrder::ExpandingCircle => self.distance * self.percent_completed,
            _ => self.distance * self.percent_completed,
        }
    }
}

#[derive(Debug)]
pub struct RecoverInfo {
    pub started_at: Instant,
    pub time_to_complete: u128,
}
