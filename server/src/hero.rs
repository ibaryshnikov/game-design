use std::time::Instant;

use nalgebra::{Point2, Vector2};

use shared::attack::{AttackInfo, RecoverInfo, SelectionInfo};
use shared::character::Character;
use shared::check_hit;
use shared::hero::{DashCooldown, DashInfo};
use shared::types::Move;

use crate::boss::Boss;
use crate::moves::Moving;
use crate::npc::Npc;

#[derive(Debug)]
pub struct Hero {
    pub id: String,
    pub hp: i32,
    max_hp: i32,
    pub position: Point2<f32>,
    direction: Vector2<f32>,
    moving: Moving,
    last_tick: Instant,
    pub melee_attack_distance: f32,
    pub ranged_attack_distance: f32,
    pub selected: Option<SelectionInfo>,
    pub attacking: Option<AttackInfo>,
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
    pub fn new(id: String, position: Point2<f32>) -> Self {
        Hero {
            id,
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
    fn is_moving(&self) -> bool {
        let moving_x = self.moving.left ^ self.moving.right;
        let moving_y = self.moving.up ^ self.moving.down;
        moving_x || moving_y
    }
    fn get_direction(&self) -> Vector2<f32> {
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
        direction
    }
    pub fn handle_move_keydown(&mut self, movement: Move) {
        match movement {
            Move::Left => self.moving.left = true,
            Move::Right => self.moving.right = true,
            Move::Up => self.moving.up = true,
            Move::Down => self.moving.down = true,
        }
        if self.is_moving() {
            self.direction = self.get_direction();
        }
    }
    pub fn handle_move_keyup(&mut self, movement: Move) {
        match movement {
            Move::Left => self.moving.left = false,
            Move::Right => self.moving.right = false,
            Move::Up => self.moving.up = false,
            Move::Down => self.moving.down = false,
        }
        if self.is_moving() {
            self.direction = self.get_direction();
        }
    }
    pub fn update(&mut self, npc: &mut [Boss]) {
        self.update_position();
        if let Some(cooldown) = &self.dash_cooldown {
            if cooldown.started.elapsed().as_millis() >= cooldown.duration {
                self.dash_cooldown = None;
            }
        }
        if self.attacking.is_some() {
            self.update_attack(npc);
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
    fn update_attack(&mut self, npc_list: &mut [Boss]) {
        let Some(attack_info) = &mut self.attacking else {
            return;
        };
        attack_info.update();
        if attack_info.completed() {
            for boss in npc_list.iter_mut() {
                if check_hit(attack_info, self.melee_attack_distance, boss.position) {
                    boss.receive_damage();
                }
            }
            let recover_info = RecoverInfo {
                started_at: Instant::now(),
                time_to_complete: 0,
            };
            self.recovering = Some(recover_info);
            self.attacking = None;
        }
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
}
