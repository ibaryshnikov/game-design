use nalgebra::{Point2, Vector2};

use network::client::{KeyActionKind, Move};
use network::server;
use shared::action::Action;
use shared::attack::{AttackInfo, AttackKind, AttackOrder, AttackState, RecoverInfo};
use shared::character::{Character, CharacterSettings};
use shared::hero::{DashCooldown, DashInfo, Moving};

use crate::boss::Boss;

#[derive(Debug, Clone)]
pub struct Hero {
    pub id: u128,
    pub hp: i32,
    max_hp: i32,
    pub position: Point2<f32>,
    pub size: f32,
    pub direction: Vector2<f32>,
    moving: Moving,
    last_key_up: Option<u128>,
    pub melee_attack_distance: f32,
    pub ranged_attack_distance: f32,
    pub action: Action,
    pub character_settings: CharacterSettings,
}

impl Character for Hero {
    fn receive_damage(&mut self) {
        if let Action::Dash(_) = self.action {
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
    fn get_position(&self) -> Point2<f32> {
        self.position
    }
    fn get_size(&self) -> f32 {
        self.size
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_character_settings_by_id(id: u32) -> CharacterSettings {
    let file_path = format!("../data/character/character_{id}.json");
    let contents = std::fs::read(file_path).expect("Should read CharacterSettings from a file");
    serde_json::from_slice(&contents).expect("Should decode CharacterSettings")
}

#[cfg(target_arch = "wasm32")]
fn load_character_settings_by_id(_id: u32) -> CharacterSettings {
    CharacterSettings::default()
}

impl Hero {
    pub fn new(id: u128, position: Point2<f32>) -> Self {
        Hero {
            id,
            hp: 1000,
            max_hp: 1000,
            position,
            size: 20.0,
            direction: Vector2::new(0.0, 0.0),
            moving: Moving {
                left: false,
                right: false,
                up: false,
                down: false,
            },
            last_key_up: None,
            melee_attack_distance: 100.0,
            ranged_attack_distance: 300.0,
            action: Action::Empty,
            character_settings: load_character_settings_by_id(1),
        }
    }
    pub fn to_network(&self) -> server::Hero {
        server::Hero {
            id: self.id,
            hp: self.hp,
            max_hp: self.max_hp,
            position: self.position,
            direction: self.direction,
            moving: self.moving.clone(),
            melee_attack_distance: self.melee_attack_distance,
            ranged_attack_distance: self.ranged_attack_distance,
            action: self.action.clone(),
            character_settings: self.character_settings.clone(),
        }
    }
    pub fn from_network(hero: server::Hero) -> Self {
        Self {
            id: hero.id,
            hp: hero.hp,
            max_hp: hero.max_hp,
            position: hero.position,
            size: 20.0,
            direction: hero.direction,
            moving: hero.moving.clone(),
            last_key_up: None,
            melee_attack_distance: hero.melee_attack_distance,
            ranged_attack_distance: hero.ranged_attack_distance,
            action: hero.action.clone(),
            character_settings: hero.character_settings.clone(),
        }
    }
    pub fn update_from_network(&mut self, hero: server::Hero) {
        self.hp = hero.hp;
        self.max_hp = hero.max_hp;
        self.position = hero.position;
        self.direction = hero.direction;
        self.moving = hero.moving.clone();
        self.melee_attack_distance = hero.melee_attack_distance;
        self.ranged_attack_distance = hero.ranged_attack_distance;
        self.action = hero.action;
        self.character_settings = hero.character_settings;
    }
    pub fn reset(&mut self) {
        self.hp = self.max_hp;
        self.position = Point2::new(100.0, 100.0);
        // self.stop();
    }
    pub fn stop(&mut self) {
        self.action = Action::Empty;
    }
    pub fn hp_left_percent(&self) -> f32 {
        self.hp as f32 / self.max_hp as f32
    }
    pub fn defeated(&self) -> bool {
        self.hp <= 0
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
        if self.action.is_some() {
            return;
        }
        let mut direction = self.direction;
        if direction.x.abs() < 0.000_001 && direction.y.abs() < 0.000_001 {
            // no direction, x & y are 0
            return;
        }
        direction.normalize_mut();
        let dash = DashInfo::new(direction, self.character_settings.dash_duration);
        self.action = Action::Dash(dash);
    }
    pub fn check_attack(&mut self) {
        if self.action.is_some() {
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

        let attack = AttackInfo {
            position: self.position,
            direction,
            delay: 50,
            time_passed: 0,
            time_to_complete: 100,
            aftercast: 0,
            percent_completed: 0.0,
            kind: AttackKind::Pizza,
            order: AttackOrder::LeftToRight,
            distance: self.melee_attack_distance,
            width_angle: 1.0,
            state: AttackState::Selected,
            damage_done: false,
        };
        self.action = Action::Attack(attack);
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
            }
            KeyActionKind::Released => {
                *moving = false;
                self.last_key_up = Some(0);
            }
        };
        self.update_direction();
    }
    pub fn update_visuals(&mut self, dt: u128) {
        self.update_position(dt);
        self.update_action_visuals(dt);
        if let Some(time_passed) = &mut self.last_key_up {
            *time_passed += dt;
            if *time_passed > 100 {
                self.last_key_up = None;
                if self.is_moving() {
                    self.update_direction();
                }
            }
        }
    }
    fn update_action(&mut self, npc: &mut [Boss], dt: u128) {
        match &mut self.action {
            Action::Attack(attack) => {
                attack.update(dt);
                if !attack.damage_done {
                    attack.check_damage_for_hero(self.melee_attack_distance, npc);
                }
                if attack.completed() {
                    self.action = Action::Recovery(RecoverInfo::new(0));
                }
            }
            Action::ComplexAttack(_attack) => {}
            Action::Dash(dash) => {
                dash.update(dt);
                let speed =
                    self.character_settings.dash_distance as f32 / dash.time_to_complete as f32;
                self.position += dash.direction * speed * 10.0;
                if dash.completed() {
                    self.action = Action::DashCooldown(DashCooldown::new(200));
                }
            }
            other => other.update(dt),
        }
    }
    fn update_action_visuals(&mut self, dt: u128) {
        match &mut self.action {
            Action::Attack(attack) => {
                attack.update(dt);
                if attack.completed() {
                    self.action = Action::Recovery(RecoverInfo::new(0));
                }
            }
            Action::ComplexAttack(_attack) => {}
            Action::Dash(dash) => {
                dash.update(dt);
                let speed =
                    self.character_settings.dash_distance as f32 / dash.time_to_complete as f32;
                self.position += dash.direction * speed * 10.0;
                if dash.completed() {
                    self.action = Action::DashCooldown(DashCooldown::new(200));
                }
            }
            other => other.update(dt),
        }
    }
    pub fn update(&mut self, npc: &mut [Boss], dt: u128) {
        self.update_position(dt);
        self.update_action(npc, dt);
        if let Some(time_passed) = &mut self.last_key_up {
            *time_passed += dt;
            if *time_passed > 100 {
                self.last_key_up = None;
                if self.is_moving() {
                    self.update_direction();
                }
            }
        }
    }
    fn update_position(&mut self, dt: u128) {
        if self.action.is_some() {
            return;
        }

        let speed = 0.1;
        if self.moving.left && self.moving.right {
            // do nothing
        } else if self.moving.left {
            self.position.x -= dt as f32 * speed;
        } else if self.moving.right {
            self.position.x += dt as f32 * speed;
        }

        if self.moving.up && self.moving.down {
            // do nothing
        } else if self.moving.up {
            self.position.y -= dt as f32 * speed;
        } else if self.moving.down {
            self.position.y += dt as f32 * speed;
        }
    }
}
