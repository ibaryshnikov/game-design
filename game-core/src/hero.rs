use nalgebra::{Point2, Vector2};

use network::client::{KeyActionKind, Move};
use network::server;
use shared::action::Action;
use shared::attack::{
    AttackInfo, AttackKind, AttackOrder, AttackState, RecoverInfo, SelectionInfo,
};
use shared::character::{Character, CharacterSettings};
use shared::check_hit;
use shared::hero::{DashCooldown, DashInfo, Moving};

use crate::boss::Boss;

#[derive(Debug, Clone)]
pub struct Hero {
    pub id: u128,
    pub hp: i32,
    max_hp: i32,
    pub position: Point2<f32>,
    pub direction: Vector2<f32>,
    moving: Moving,
    last_key_up: Option<u128>,
    pub melee_attack_distance: f32,
    pub ranged_attack_distance: f32,
    pub selected: Option<SelectionInfo>,
    pub attacking: Option<AttackInfo>,
    pub recovering: Option<RecoverInfo>,
    pub dashing: Option<DashInfo>,
    dash_cooldown: Option<DashCooldown>,
    pub action: Option<Action>,
    character_settings: CharacterSettings,
}

impl Character for Hero {
    fn get_recovering_state(&mut self) -> &mut Option<RecoverInfo> {
        &mut self.recovering
    }
    fn clear_recovering_state(&mut self) {
        self.recovering = None;
    }
}

fn load_character_settings_by_id(id: u32) -> CharacterSettings {
    let file_path = format!("../data/character/character_{}.json", id);
    let contents = std::fs::read(file_path).expect("Should read CharacterSettings from a file");
    serde_json::from_slice(&contents).expect("Should decode CharacterSettings")
}

impl Hero {
    pub fn new(id: u128, position: Point2<f32>) -> Self {
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
            last_key_up: None,
            melee_attack_distance: 100.0,
            ranged_attack_distance: 300.0,
            selected: None,
            attacking: None,
            recovering: None,
            dashing: None,
            dash_cooldown: None,
            action: None,
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
            selected: self.selected.clone(),
            attacking: self.attacking.clone(),
            recovering: self.recovering.clone(),
            dashing: self.dashing.clone(),
            dash_cooldown: self.dash_cooldown.clone(),
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
            direction: hero.direction,
            moving: hero.moving.clone(),
            last_key_up: None,
            melee_attack_distance: hero.melee_attack_distance,
            ranged_attack_distance: hero.ranged_attack_distance,
            selected: hero.selected.clone(),
            attacking: hero.attacking.clone(),
            recovering: hero.recovering.clone(),
            dashing: hero.dashing.clone(),
            dash_cooldown: hero.dash_cooldown.clone(),
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
        self.selected = hero.selected;
        self.attacking = hero.attacking;
        self.recovering = hero.recovering;
        self.dashing = hero.dashing;
        self.dash_cooldown = hero.dash_cooldown;
        self.action = hero.action;
        self.character_settings = hero.character_settings;
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
        let dash_info = DashInfo::new(direction, self.character_settings.dash_duration);
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
        self.attacking = Some(attack_info);
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
        if let Some(time_passed) = &mut self.last_key_up {
            *time_passed += dt;
            if *time_passed > 100 {
                self.last_key_up = None;
                if self.is_moving() {
                    self.update_direction();
                }
            }
        }
        if let Some(cooldown) = &mut self.dash_cooldown {
            cooldown.update(dt);
            if cooldown.completed() {
                self.dash_cooldown = None;
            }
        }
        if self.attacking.is_some() {
            self.update_attack_visuals(dt);
        }
        if self.recovering.is_some() {
            self.update_recovery(dt);
        }
    }
    pub fn update(&mut self, npc: &mut [Boss], dt: u128) {
        self.update_position(dt);
        if let Some(time_passed) = &mut self.last_key_up {
            *time_passed += dt;
            if *time_passed > 100 {
                self.last_key_up = None;
                if self.is_moving() {
                    self.update_direction();
                }
            }
        }
        if let Some(cooldown) = &mut self.dash_cooldown {
            cooldown.update(dt);
            if cooldown.completed() {
                self.dash_cooldown = None;
            }
        }
        if self.attacking.is_some() {
            self.update_attack(npc, dt);
        }
        if self.recovering.is_some() {
            self.update_recovery(dt);
        }
    }
    fn update_position(&mut self, dt: u128) {
        if self.attacking.is_some() {
            return;
        }
        if self.recovering.is_some() {
            return;
        }

        if let Some(dash_info) = &mut self.dashing {
            dash_info.update(dt);
            if dash_info.completed() {
                self.position += dash_info.direction * self.character_settings.dash_distance as f32;
                self.dashing = None;
                self.dash_cooldown = Some(DashCooldown::new(200));
            }
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
    fn update_attack_visuals(&mut self, dt: u128) {
        let Some(attack_info) = &mut self.attacking else {
            return;
        };
        attack_info.update(dt);
        if attack_info.completed() {
            self.recovering = Some(RecoverInfo::new(0));
            self.attacking = None;
        }
    }
    fn update_attack(&mut self, npc: &mut [Boss], dt: u128) {
        let Some(attack_info) = &mut self.attacking else {
            return;
        };
        attack_info.update(dt);
        if attack_info.completed() {
            for boss in npc.iter_mut() {
                if check_hit(attack_info, self.melee_attack_distance, boss.position) {
                    boss.receive_damage();
                }
            }
            self.recovering = Some(RecoverInfo::new(0));
            self.attacking = None;
        }
    }
}
