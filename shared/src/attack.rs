use std::fmt::{self, Display};
use std::time::Instant;

use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SelectionInfo {
    pub position: Point2<f32>,
    pub selected_at: Instant,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MissileShape {
    Circle,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MissileInfo {
    pub number_of_missiles: u8,
    pub shape: MissileShape,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum AttackKind {
    Pizza,
    Circle,
    // Missiles(Box<MissileInfo>),
}

impl AttackKind {
    pub fn options() -> [AttackKind; 2] {
        use AttackKind::*;
        [Pizza, Circle]
    }
}

impl Display for AttackKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

impl AttackOrder {
    pub const fn options() -> [AttackOrder; 9] {
        use AttackOrder::*;
        [
            LeftToRight,
            RightToLeft,
            LeftThenRight,
            RightThenLeft,
            CloseToFar,
            CenterToSides,
            SidesToCenter,
            ExpandingCircle,
            ProjectileFromCaster,
        ]
    }
}

impl Display for AttackOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AttackState {
    Selected,
    Attacking,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackConstructor {
    pub name: String,
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub delay: u128,
    pub time_to_complete: u128, // ms
    pub aftercast: u128,
    pub kind: AttackKind,
    pub order: AttackOrder,
    pub range: AttackRange,
    pub width_angle: f32,
    pub state: AttackState,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackRange {
    pub from: f32,
    pub to: f32,
}

impl AttackRange {
    pub fn in_range(&self, value: f32) -> bool {
        self.from <= value && value <= self.to
    }
}

impl AttackConstructor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl Default for AttackConstructor {
    fn default() -> Self {
        Self {
            name: String::new(),
            position: Point2::default(),
            direction: Vector2::default(),
            delay: 0,
            time_to_complete: 0,
            aftercast: 0,
            kind: AttackKind::Pizza,
            order: AttackOrder::CloseToFar,
            range: AttackRange::default(),
            width_angle: 0.0,
            state: AttackState::Selected,
        }
    }
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
    pub width_angle: f32,
    pub state: AttackState,
    pub damage_done: bool,
}

impl AttackInfo {
    pub fn from_constructor(
        constructor: AttackConstructor,
        position: Point2<f32>,
        direction: Vector2<f32>,
        distance: f32,
    ) -> Self {
        let AttackConstructor {
            // position,
            // direction,
            delay,
            time_to_complete,
            aftercast,
            kind,
            order,
            width_angle,
            state,
            ..
        } = constructor;
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay,
            time_to_complete,
            aftercast,
            percent_completed: 0.0,
            kind,
            order,
            distance,
            width_angle,
            state,
            damage_done: false,
        }
    }
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
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
        }
    }
    pub fn narrow(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 100,
            aftercast: 500,
            percent_completed: 0.0,
            kind: AttackKind::Pizza,
            order: AttackOrder::CloseToFar,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
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
            kind: AttackKind::Pizza,
            order: AttackOrder::LeftToRight,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
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
            kind: AttackKind::Pizza,
            order: AttackOrder::RightToLeft,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
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
            kind: AttackKind::Pizza,
            order: AttackOrder::LeftThenRight,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
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
            kind: AttackKind::Pizza,
            order: AttackOrder::RightThenLeft,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
        }
    }
    pub fn split(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Pizza,
            order: AttackOrder::CenterToSides,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
        }
    }
    pub fn closing(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 300,
            time_to_complete: 200,
            aftercast: 600,
            percent_completed: 0.0,
            kind: AttackKind::Pizza,
            order: AttackOrder::SidesToCenter,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
        }
    }
    pub fn fireball(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        let position = Point2::new(
            position.x + direction.x * 70.0,
            position.y + direction.y * 70.0,
        );
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 100,
            time_to_complete: 500,
            aftercast: 300,
            percent_completed: 0.0,
            kind: AttackKind::Circle,
            order: AttackOrder::ProjectileFromCaster,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
        }
    }
    pub fn fireblast(position: Point2<f32>, direction: Vector2<f32>, distance: f32) -> Self {
        AttackInfo {
            position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 400,
            time_to_complete: 600,
            aftercast: 300,
            percent_completed: 0.0,
            kind: AttackKind::Circle,
            order: AttackOrder::ExpandingCircle,
            distance,
            width_angle: 0.0,
            state: AttackState::Selected,
            damage_done: false,
        }
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
                // self.time_passed = time_passed;
                let mut percent_completed = time_passed as f32 / self.time_to_complete as f32;
                if percent_completed > 1.0 {
                    percent_completed = 1.0;
                }
                self.percent_completed = percent_completed;
                if let AttackOrder::ProjectileFromCaster = self.order {
                    self.position.x += self.direction.x * time_passed as f32 / 30.0;
                    self.position.y += self.direction.y * time_passed as f32 / 30.0;
                }
            }
        }
    }
    pub fn completed(&self) -> bool {
        let time_passed = self.started_at.elapsed().as_millis();
        time_passed > self.time_to_complete + self.aftercast
    }
    pub fn width_radian(&self) -> f32 {
        let radian = self.width_angle;
        if let AttackOrder::RightToLeft | AttackOrder::RightThenLeft = self.order {
            -radian
        } else {
            radian
        }
    }
    pub fn get_base_angle(&self) -> f32 {
        self.direction.y.atan2(self.direction.x) + std::f32::consts::PI
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
            AttackOrder::ProjectileFromCaster => self.distance,
        }
    }
}

#[derive(Debug)]
pub struct RecoverInfo {
    pub started_at: Instant,
    pub time_to_complete: u128,
}
