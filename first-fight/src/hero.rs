use std::time::Instant;

use iced_core::{Color, Size};
use iced_widget::canvas::{stroke, Frame, Path, Stroke};
use nalgebra::{Point2, Vector2};

use shared::attack::{
    AttackInfo, AttackKind, AttackOrder, AttackState, RecoverInfo, SelectionInfo,
};
use shared::character::Character;
use shared::check_hit;
use shared::hero::{DashCooldown, DashInfo};
use shared::types::KeyActionKind;

use crate::attack::AttackView;
use crate::boss::Boss;
use crate::Move;

struct Moving {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

pub struct Hero {
    pub hp: i32,
    max_hp: i32,
    pub position: Point2<f32>,
    direction: Vector2<f32>,
    moving: Moving,
    last_key_up: Option<Instant>,
    last_tick: Instant,
    pub melee_attack_distance: f32,
    pub ranged_attack_distance: f32,
    pub selected: Option<SelectionInfo>,
    pub attacking: Option<AttackView>,
    pub recovering: Option<RecoverInfo>,
    dashing: Option<DashInfo>,
    dash_cooldown: Option<DashCooldown>,
}

impl Character for Hero {
    fn get_recovering_state(&mut self) -> &mut Option<RecoverInfo> {
        &mut self.recovering
    }
    fn clear_recovering_state(&mut self) {
        self.recovering = None;
    }
}

impl Hero {
    pub fn new(position: Point2<f32>) -> Self {
        Hero {
            hp: 1000,
            max_hp: 1000,
            position,
            direction: Vector2::new(0.0, 0.0),
            moving: Moving {
                left: false,
                right: false,
                up: false,
                down: false,
            },
            last_key_up: None,
            last_tick: Instant::now(),
            melee_attack_distance: 100.0,
            ranged_attack_distance: 300.0,
            selected: None,
            attacking: None,
            recovering: None,
            dashing: None,
            dash_cooldown: None,
        }
    }
    pub fn reset(&mut self) {
        self.hp = self.max_hp;
        self.position = Point2::new(100.0, 100.0);
        // self.stop();
    }
    pub fn stop(&mut self) {
        self.selected = None;
        self.attacking = None;
        self.recovering = None;
    }
    pub fn receive_damage(&mut self) {
        if self.dashing.is_some() {
            // invulnerability frame
            return;
        }
        if self.hp == 0 {
            return;
        }
        self.hp -= 35;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    fn hp_left_percent(&self) -> f32 {
        self.hp as f32 / self.max_hp as f32
    }
    pub fn draw_body(&self, frame: &mut Frame) {
        if let Some(dash_info) = &self.dashing {
            let percent_completed = dash_info.percent_completed();
            let position = iced_core::Point::new(
                self.position.x + dash_info.direction.x * 150.0 * percent_completed,
                self.position.y + dash_info.direction.y * 150.0 * percent_completed,
            );
            let path = Path::new(|b| {
                b.circle(position, 20.0);
            });
            frame.stroke(
                &path,
                Stroke {
                    style: stroke::Style::Solid(Color::BLACK),
                    width: 3.0,
                    ..Stroke::default()
                },
            );
            return;
        }
        let path = Path::new(|b| {
            b.circle(
                iced_core::Point::new(self.position.x, self.position.y),
                20.0,
            );
        });
        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(Color::BLACK),
                width: 3.0,
                ..Stroke::default()
            },
        );
        self.draw_direction(frame);
    }
    fn draw_direction(&self, frame: &mut Frame) {
        let mut direction = self.direction;
        if direction.x.abs() < 0.000_001 && direction.y.abs() < 0.000_001 {
            // no direction, x & y are 0
        } else {
            direction.normalize_mut();
        }
        let start = iced_core::Point::new(
            self.position.x + direction.x * 10.0,
            self.position.y + direction.y * 10.0,
        );
        let path = Path::new(|b| {
            b.circle(start, 5.0);
        });

        frame.fill(&path, Color::from_rgb8(0, 255, 0));
    }
    pub fn draw_health_bar(&self, frame: &mut Frame) {
        // self.draw_test_data(frame);
        let start = iced_core::Point::new(10.0, 10.0);
        let bar_width = 200.0;
        let bar_height = 20.0;

        // draw red background
        let path = Path::new(|b| {
            let size = Size::new(bar_width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(255, 0, 0));

        // draw hp left as green
        let path = Path::new(|b| {
            let width = bar_width * self.hp_left_percent();
            let size = Size::new(width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(0, 255, 0));
    }
    fn draw_test_data(&self, frame: &mut Frame) {
        let point_a = iced_core::Point::new(512.0, 384.0);
        let point_b = iced_core::Point::new(362.60272, 370.567);
        let point_c = iced_core::Point::new(379.62704, 313.4493);
        let center = iced_core::Point::new(402.8994, 376.09946);
        let radius = 20.0;
        let path = Path::new(|b| {
            b.move_to(point_a);
            b.line_to(point_b);
            b.line_to(point_c);
            b.line_to(point_a);
            b.move_to(center);
            b.circle(center, radius);
        });
        frame.fill(&path, Color::from_rgb8(0, 0, 255));
    }
    fn is_moving(&self) -> bool {
        let moving_x = self.moving.left ^ self.moving.right;
        let moving_y = self.moving.up ^ self.moving.down;
        moving_x || moving_y
    }
    fn update_direction(&mut self) {
        let mut direction = Vector2::new(0.0, 0.0);
        if self.moving.up {
            direction.y -= 1.0;
        }
        if self.moving.down {
            direction.y += 1.0;
        }
        if self.moving.left {
            direction.x -= 1.0;
        }
        if self.moving.right {
            direction.x += 1.0;
        }
        self.direction = direction;
    }
    pub fn dash(&mut self) {
        if self.attacking.is_some() {
            return;
        }
        if self.recovering.is_some() {
            return;
        }
        if self.dashing.is_some() || self.dash_cooldown.is_some() {
            return;
        }
        let mut direction = self.direction;
        if direction.x.abs() < 0.000_001 && direction.y.abs() < 0.000_001 {
            // no direction, x & y are 0
            return;
        }
        direction.normalize_mut();
        let dash_info = DashInfo {
            direction,
            started: Instant::now(),
            time_to_complete: 100,
        };
        self.dashing = Some(dash_info);
    }
    pub fn check_attack(&mut self) {
        if self.attacking.is_some() {
            return;
        }
        if self.recovering.is_some() {
            return;
        }
        let mut direction = self.direction;
        direction.x = -direction.x;
        direction.y = -direction.y;
        if direction.x.abs() < 0.000_001 && direction.y.abs() < 0.000_001 {
            // no direction, x & y are 0
            return;
        }
        direction.normalize_mut();

        let attack_info = AttackInfo {
            position: self.position,
            direction,
            started_at: Instant::now(),
            time_passed: 0,
            delay: 50,
            time_to_complete: 100,
            aftercast: 0,
            percent_completed: 0.0,
            kind: AttackKind::CustomAngle(1.0),
            order: AttackOrder::LeftToRight,
            distance: self.melee_attack_distance,
            state: AttackState::Selected,
        };
        self.attacking = Some(AttackView::new(attack_info));
    }
    pub fn handle_move_action(&mut self, kind: KeyActionKind, movement: Move) {
        let moving = match movement {
            Move::Left => &mut self.moving.left,
            Move::Right => &mut self.moving.right,
            Move::Up => &mut self.moving.up,
            Move::Down => &mut self.moving.down,
        };
        match kind {
            KeyActionKind::Pressed => {
                *moving = true;
                self.update_direction();
            }
            KeyActionKind::Released => {
                *moving = false;
                self.last_key_up = Some(Instant::now());
            }
        };
    }
    pub fn update(&mut self, boss: &mut Boss) {
        self.update_position();
        if let Some(time_passed) = self.last_key_up {
            if time_passed.elapsed().as_millis() > 100 {
                self.last_key_up = None;
                if self.is_moving() {
                    self.update_direction();
                }
            }
        }
        if let Some(cooldown) = &self.dash_cooldown {
            if cooldown.started.elapsed().as_millis() >= cooldown.duration {
                self.dash_cooldown = None;
            }
        }
        if self.attacking.is_some() {
            self.update_attack(boss);
        }
        if self.recovering.is_some() {
            self.update_recovery();
        }
    }
    fn update_position(&mut self) {
        let now = Instant::now();
        let elapsed = self.last_tick.elapsed().as_millis();
        self.last_tick = now;

        if self.attacking.is_some() {
            return;
        }
        if self.recovering.is_some() {
            return;
        }

        if let Some(dash_info) = &self.dashing {
            if dash_info.percent_completed() > 1.0 {
                self.position += dash_info.direction * 150.0;
                self.dashing = None;
                let cooldown = DashCooldown {
                    started: Instant::now(),
                    duration: 200,
                };
                self.dash_cooldown = Some(cooldown);
            }
        }

        let speed = 0.1;
        if self.moving.left && self.moving.right {
            // do nothing
        } else if self.moving.left {
            self.position.x -= elapsed as f32 * speed;
        } else if self.moving.right {
            self.position.x += elapsed as f32 * speed;
        }

        if self.moving.up && self.moving.down {
            // do nothing
        } else if self.moving.up {
            self.position.y -= elapsed as f32 * speed;
        } else if self.moving.down {
            self.position.y += elapsed as f32 * speed;
        }
    }
    fn update_attack(&mut self, boss: &mut Boss) {
        let Some(attack_view) = &mut self.attacking else {
            return;
        };
        attack_view.attack_info.update();
        if attack_view.attack_info.completed() {
            if check_hit(
                &attack_view.attack_info,
                self.melee_attack_distance,
                boss.position,
            ) {
                boss.receive_damage();
            }
            let recover_info = RecoverInfo {
                started_at: Instant::now(),
                time_to_complete: 0,
            };
            self.recovering = Some(recover_info);
            self.attacking = None;
        }
    }
    pub fn draw_attack(&self, frame: &mut Frame) {
        if let Some(attack) = &self.attacking {
            attack.draw(frame);
        }
    }
}
