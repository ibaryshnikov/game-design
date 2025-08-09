use std::collections::HashMap;
use std::fmt::{self, Display};

use nalgebra::{Point2, Vector2};
use serde::{Deserialize, Serialize};

use crate::character::Character;
use crate::check_hit;

pub trait ReceiveDamage {
    fn receive_damage(&mut self, value: u32);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComplexAttackConstructor {
    pub range: AttackRange,
    pub sequences: Vec<AttackSequenceConstructor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ComplexAttack {
    pub time_passed: u128,
    pub sequences: Vec<AttackSequence>,
    pub attacker_position: Point2<f32>,
    pub target_position: Point2<f32>,
    pub direction_angle: f32,
}

impl ComplexAttack {
    pub fn from_constructor(
        constructor: ComplexAttackConstructor,
        attacker_position: Point2<f32>,
        target_position: Point2<f32>,
        direction: Vector2<f32>,
        direction_angle: f32,
    ) -> Self {
        let sequences = constructor
            .sequences
            .iter()
            .map(|item| {
                AttackSequence::from_constructor(
                    item.clone(),
                    attacker_position,
                    target_position,
                    direction,
                    direction_angle,
                )
            })
            .collect();
        Self {
            time_passed: 0,
            sequences,
            attacker_position,
            target_position,
            direction_angle,
        }
    }
    pub fn update(&mut self, dt: u128) {
        for sequence in self.sequences.iter_mut() {
            sequence.update(dt);
        }
    }
    pub fn completed(&self) -> bool {
        self.sequences.iter().all(|item| item.completed())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackSequenceConstructor {
    pub position_offset: Point2<f32>,
    pub parts: Vec<AttackPartConstructor>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackSequence {
    pub time_passed: u128,
    pub parts: Vec<AttackPart>,
    pub index: usize,
    // pub active_part: Option<AttackPart>,
}

impl AttackSequence {
    pub fn from_constructor(
        constructor: AttackSequenceConstructor,
        attacker_position: Point2<f32>,
        target_position: Point2<f32>,
        direction: Vector2<f32>,
        direction_angle: f32,
    ) -> Self {
        let parts = constructor
            .parts
            .iter()
            .map(|item| {
                AttackPart::from_constructor(
                    item.clone(),
                    attacker_position,
                    target_position,
                    constructor.position_offset,
                    direction,
                    direction_angle,
                )
            })
            .collect();
        Self {
            time_passed: 0,
            parts,
            index: 0,
            // active_part: None,
        }
    }
    pub fn active_part(&self) -> Option<&AttackPart> {
        self.parts.get(self.index)
    }
    pub fn update(&mut self, dt: u128) {
        if self.parts.is_empty() {
            return;
        }
        self.update_with_index(dt);
    }
    fn update_with_index(&mut self, dt: u128) {
        if self.index >= self.parts.len() {
            return;
        }
        if self.parts[self.index].completed() {
            self.index += 1;
            self.update_with_index(dt);
        } else {
            self.parts[self.index].update(dt);
        }
    }
    pub fn completed(&self) -> bool {
        // self.parts.iter().all(|item| item.completed())
        // double increment paranoia
        self.index >= self.parts.len()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CircleConstructor {
    pub radius: f32,
    pub time_to_complete: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Circle {
    pub time_passed: u128,
    pub time_to_complete: u128,
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub radius: f32,
    pub percent_completed: f32,
}

impl Circle {
    pub fn from_constructor(
        constructor: CircleConstructor,
        position: Point2<f32>,
        direction: Vector2<f32>,
    ) -> Self {
        Self {
            time_passed: 0,
            time_to_complete: constructor.time_to_complete,
            position,
            direction,
            radius: constructor.radius,
            percent_completed: 0.0,
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
        let mut percent_completed = self.time_passed as f32 / self.time_to_complete as f32;
        if percent_completed > 1.0 {
            percent_completed = 1.0;
        }
        self.percent_completed = percent_completed;
        self.position.x += self.direction.x * self.time_passed as f32 / 30.0;
        self.position.y += self.direction.y * self.time_passed as f32 / 30.0;
    }
    pub fn intersects_with_circle(&self, center: Point2<f32>, radius: f32) -> bool {
        let dx = self.position.x - center.x;
        let dy = self.position.y - center.y;
        let dd = (dx * dx + dy * dy).sqrt();
        dd < self.radius + radius
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PizzaConstructor {
    pub radius: f32,
    pub width_angle: f32,
    pub order: AttackOrder,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pizza {
    pub time_passed: u128,
    pub time_to_complete: u128,
    pub position: Point2<f32>,
    pub radius: f32,
    pub direction: Vector2<f32>,
    pub width_angle: f32,
    pub order: AttackOrder,
    pub percent_completed: f32,
}

pub enum PizzaAnimationKind {
    StartAngle,
    EndAngle,
    WidthAngle,
}

impl Pizza {
    pub fn from_constructor(
        constructor: PizzaConstructor,
        position: Point2<f32>,
        direction: Vector2<f32>,
    ) -> Self {
        let PizzaConstructor {
            radius,
            width_angle,
            order,
        } = constructor;
        Self {
            time_passed: 0,
            time_to_complete: 0,
            position,
            radius,
            direction,
            width_angle,
            order,
            percent_completed: 0.0,
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
        // do nothing for now
    }
    pub fn intersects_with_circle(&self, center: Point2<f32>, radius: f32) -> bool {
        let angle = self.get_base_angle();

        let (start_angle, end_angle) = self.get_angles(angle, self.width_radian());

        let point_a = self.position;
        let point_b = Point2::new(
            point_a.x + self.radius * start_angle.cos(),
            point_a.y + self.radius * start_angle.sin(),
        );
        let point_c = Point2::new(
            point_a.x + self.radius * end_angle.cos(),
            point_a.y + self.radius * end_angle.sin(),
        );

        crate::check_points_with_circle(point_a, point_b, point_c, center, radius)
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
        let percent_completed = self.percent_completed;
        match self.order {
            AttackOrder::CloseToFar => self.radius * percent_completed,
            AttackOrder::LeftToRight => self.radius,
            AttackOrder::RightToLeft => self.radius,
            AttackOrder::SidesToCenter => self.radius,
            AttackOrder::CenterToSides => self.radius,
            AttackOrder::LeftThenRight => self.radius,
            AttackOrder::RightThenLeft => self.radius,
            AttackOrder::ExpandingCircle => self.radius * percent_completed,
            AttackOrder::ProjectileFromCaster => self.radius,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AttackShapeConstructor {
    Circle(CircleConstructor),
    Pizza(PizzaConstructor),
    Ellipse,
    Triangle,
    Rectangle,
    Hexagon,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AttackShape {
    Circle(Circle),
    Pizza(Pizza),
    Ellipse,
    Triangle,
    Rectangle,
    Hexagon,
}

impl AttackShape {
    pub fn from_constructor(
        constructor: AttackShapeConstructor,
        attacker_position: Point2<f32>,
        _target_position: Point2<f32>,
        position_offset: Point2<f32>,
        direction: Vector2<f32>,
    ) -> Self {
        match constructor {
            AttackShapeConstructor::Circle(circle) => {
                let x = attacker_position.x + position_offset.x;
                let y = attacker_position.y + position_offset.y;
                let position = Point2::new(x, y);
                AttackShape::Circle(Circle::from_constructor(circle, position, direction))
            }
            AttackShapeConstructor::Pizza(pizza) => {
                AttackShape::Pizza(Pizza::from_constructor(pizza, attacker_position, direction))
            }
            AttackShapeConstructor::Ellipse => AttackShape::Ellipse,
            AttackShapeConstructor::Triangle => AttackShape::Triangle,
            AttackShapeConstructor::Rectangle => AttackShape::Rectangle,
            AttackShapeConstructor::Hexagon => AttackShape::Hexagon,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackDamageConstructor {
    pub value: u32,
    pub instances: u32,
    pub delay_between_instances: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackDamage {
    pub value: u32,
    pub instances: u32,
    pub delay_between_instances: u32,
    pub time_since_last_done: u128,
}

impl AttackDamage {
    pub fn from_constructor(constructor: AttackDamageConstructor) -> Self {
        let AttackDamageConstructor {
            value,
            instances,
            delay_between_instances,
        } = constructor;
        Self {
            value,
            instances,
            delay_between_instances,
            time_since_last_done: 0,
        }
    }
    pub fn has_instances(&self) -> bool {
        self.instances > 0
    }
    pub fn do_damage<T: ReceiveDamage>(&mut self, target: &mut T, dt: u128) {
        if !self.has_instances() {
            return;
        }
        self.time_since_last_done += dt;
        if self.time_since_last_done < self.delay_between_instances as u128 {
            return;
        }
        target.receive_damage(self.value);
        self.instances -= 1;
        self.time_since_last_done = 0;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackPartConstructor {
    pub time_to_complete: u128,
    pub shape: AttackShapeConstructor,
    pub radius: f32,
    pub damage: Option<AttackDamageConstructor>,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackPart {
    pub time_passed: u128,
    pub time_to_complete: u128,
    pub shape: AttackShape,
    pub attacker_position: Point2<f32>,
    pub target_position: Point2<f32>,
    pub direction_angle: f32,
    pub radius: f32,
    pub damage: Option<AttackDamage>,
}

impl AttackPart {
    pub fn from_constructor(
        constructor: AttackPartConstructor,
        attacker_position: Point2<f32>,
        target_position: Point2<f32>,
        position_offset: Point2<f32>,
        direction: Vector2<f32>,
        direction_angle: f32,
    ) -> Self {
        let AttackPartConstructor {
            time_to_complete,
            shape,
            radius,
            damage,
        } = constructor;
        let damage = damage.map(AttackDamage::from_constructor);
        let shape = AttackShape::from_constructor(
            shape,
            attacker_position,
            target_position,
            position_offset,
            direction,
        );
        Self {
            time_passed: 0,
            time_to_complete,
            shape,
            attacker_position,
            target_position,
            direction_angle,
            radius,
            damage,
        }
    }
    pub fn intersects_with_circle(&self, center: Point2<f32>, radius: f32) -> bool {
        use AttackShape::*;
        match &self.shape {
            Circle(circle) => circle.intersects_with_circle(center, radius),
            Pizza(pizza) => pizza.intersects_with_circle(center, radius),
            _ => false, // not implemented yet
        }
    }
    pub fn update(&mut self, dt: u128) {
        use AttackShape::*;
        match &mut self.shape {
            Circle(circle) => circle.update(dt),
            Pizza(pizza) => pizza.update(dt),
            _ => (),
        }
    }
    pub fn completed(&self) -> bool {
        self.time_passed >= self.time_to_complete
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelectionInfo {
    pub position: Point2<f32>,
    pub time_passed: u128,
}

impl SelectionInfo {
    pub fn new(position: Point2<f32>) -> Self {
        Self {
            position,
            time_passed: 0,
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
    }
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
        write!(f, "{self:?}")
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
        write!(f, "{self:?}")
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttackInfo {
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    pub delay: u128,
    pub time_passed: u128,
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
            delay,
            time_passed: 0,
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
            delay: 700,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 300,
            time_passed: 0,
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
            delay: 100,
            time_passed: 0,
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
            delay: 400,
            time_passed: 0,
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
    pub fn check_damage_for_boss<T>(&mut self, characters: &mut HashMap<u128, T>)
    where
        T: Character,
    {
        for hero in characters.values_mut() {
            if check_hit(self, self.distance, hero.get_position(), hero.get_size()) {
                hero.receive_damage();
                self.damage_done = true;
            }
        }
    }
    pub fn check_damage_for_hero<T>(&mut self, melee_attack_distance: f32, npc: &mut [T])
    where
        T: Character,
    {
        for boss in npc.iter_mut() {
            if check_hit(
                self,
                melee_attack_distance,
                boss.get_position(),
                boss.get_size(),
            ) {
                boss.receive_damage();
                self.damage_done = true;
            }
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
        match self.state {
            AttackState::Selected => {
                if self.time_passed < self.delay {
                    return;
                }
                self.state = AttackState::Attacking;
                self.time_passed -= self.delay;
            }
            AttackState::Attacking => {
                let mut percent_completed = self.time_passed as f32 / self.time_to_complete as f32;
                if percent_completed > 1.0 {
                    percent_completed = 1.0;
                }
                self.percent_completed = percent_completed;
                if let AttackOrder::ProjectileFromCaster = self.order {
                    self.position.x += self.direction.x * self.time_passed as f32 / 30.0;
                    self.position.y += self.direction.y * self.time_passed as f32 / 30.0;
                }
            }
        }
    }
    pub fn completed(&self) -> bool {
        self.time_passed > self.time_to_complete + self.aftercast
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecoverInfo {
    pub time_passed: u128,
    pub time_to_complete: u128,
}

impl RecoverInfo {
    pub fn new(time_to_complete: u128) -> Self {
        RecoverInfo {
            time_passed: 0,
            time_to_complete,
        }
    }
    pub fn update(&mut self, dt: u128) {
        self.time_passed += dt;
    }
    pub fn completed(&self) -> bool {
        self.time_passed >= self.time_to_complete
    }
}
