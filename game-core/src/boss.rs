use std::time::Instant;

use nalgebra::Point2;
use nalgebra::Vector2;

use shared::attack::{AttackConstructor, AttackInfo, AttackOrder, RecoverInfo};
use shared::character::Character;
use shared::check_hit;
use shared::npc::{NpcConstructor, load_attacks};
use shared::position::{direction_from, distance_between};

use crate::hero::Hero;

pub struct Boss {
    pub position: Point2<f32>,
    attacks: Vec<AttackConstructor>,
    pub attacking: Option<AttackInfo>,
    recovering: Option<RecoverInfo>,
    pub hp: i32,
    max_hp: i32,
}

impl Character for Boss {
    fn get_recovering_state(&mut self) -> &mut Option<RecoverInfo> {
        &mut self.recovering
    }
    fn clear_recovering_state(&mut self) {
        self.recovering = None;
    }
}

impl Boss {
    pub fn new(position: Point2<f32>) -> Self {
        Boss {
            position,
            attacks: Vec::new(),
            attacking: None,
            recovering: None,
            hp: 300,
            max_hp: 300,
        }
    }
    pub fn from_constructor(position: Point2<f32>, constructor: NpcConstructor) -> Self {
        let NpcConstructor { attacks, .. } = constructor;
        let attacks = load_attacks(attacks);
        Boss {
            position,
            attacks,
            attacking: None,
            recovering: None,
            hp: 300,
            max_hp: 300,
        }
    }
    pub fn reset(&mut self) {
        self.hp = self.max_hp;
    }
    pub fn stop(&mut self) {
        self.attacking = None;
        self.recovering = None;
    }
    pub fn update(&mut self, hero: &mut Hero) {
        if self.attacking.is_some() {
            self.update_attack(hero);
        }
        if self.recovering.is_some() {
            self.update_recovery();
        }
        self.check_new_attack(hero.position);
    }
    fn update_attack(&mut self, hero: &mut Hero) {
        let Some(attack_info) = &mut self.attacking else {
            return;
        };
        attack_info.update();
        match attack_info.order {
            AttackOrder::ProjectileFromCaster => {
                if !attack_info.damage_done
                    && check_hit(attack_info, attack_info.range, hero.position)
                {
                    hero.receive_damage();
                    attack_info.damage_done = true;
                }
            }
            _ => (),
        }
        if attack_info.completed() {
            if let AttackOrder::ProjectileFromCaster = attack_info.order {
                // do nothing, we did check_hit above
            } else if check_hit(attack_info, attack_info.range, hero.position) {
                hero.receive_damage();
            }
            let recover_info = RecoverInfo {
                started_at: Instant::now(),
                time_to_complete: 500,
            };
            self.recovering = Some(recover_info);
            self.attacking = None;
        }
    }
    fn check_new_attack(&mut self, character_position: Point2<f32>) {
        if self.attacking.is_some() || self.recovering.is_some() {
            return;
        }
        let distance = distance_between(&self.position, &character_position);
        let attacks: Vec<_> = self
            .attacks
            .iter()
            .filter(|attack| attack.range > distance)
            .collect();
        if attacks.is_empty() {
            return;
        }
        let index = rand::random_range(0..attacks.len());
        let constructor = attacks[index].clone();

        let mut direction = direction_from(&self.position, &character_position);
        if direction.norm() > 0.000_001 {
            direction.normalize_mut();
        }
        let attack_info = match &constructor.order {
            AttackOrder::ExpandingCircle => {
                AttackInfo::from_constructor(constructor, character_position, direction, 70.0)
            }
            AttackOrder::ProjectileFromCaster => {
                let direction = Vector2::new(-direction.x, -direction.y);
                AttackInfo::from_constructor(constructor, self.position, direction, 20.0)
            }
            _ => {
                let range = constructor.range;
                AttackInfo::from_constructor(constructor, self.position, direction, range)
            }
        };
        self.attacking = Some(attack_info);
    }
    pub fn receive_damage(&mut self) {
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
}
