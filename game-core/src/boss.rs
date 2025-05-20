use std::time::Instant;

use nalgebra::{Point2, Vector2};

use shared::attack::{AttackInfo, AttackKind, AttackOrder, RecoverInfo};
use shared::character::Character;
use shared::check_hit;
use shared::npc::NpcConstructor;
use shared::position::{direction_from, distance_between};

use crate::hero::Hero;

pub struct Boss {
    pub position: Point2<f32>,
    close_melee_attack_distance: f32,
    attack_index: u8,
    pub melee_attack_distance: f32,
    ranged_attack_index: u8,
    pub ranged_attack_distance: f32,
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
            close_melee_attack_distance: 150.0,
            attack_index: 0,
            melee_attack_distance: 300.0,
            ranged_attack_index: 0,
            ranged_attack_distance: 500.0,
            attacking: None,
            recovering: None,
            hp: 300,
            max_hp: 300,
        }
    }
    pub fn from_constructor(position: Point2<f32>, constructor: NpcConstructor) -> Self {
        let NpcConstructor {
            close_melee_attack_distance,
            melee_attack_distance,
            ranged_attack_distance,
            ..
        } = constructor;
        Boss {
            position,
            close_melee_attack_distance,
            attack_index: 0,
            melee_attack_distance,
            ranged_attack_index: 0,
            ranged_attack_distance,
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
                if !attack_info.damage_done {
                    if check_hit(attack_info, attack_info.distance, hero.position) {
                        hero.receive_damage();
                        attack_info.damage_done = true;
                    }
                }
            }
            _ => (),
        }
        if attack_info.completed() {
            if let AttackOrder::ProjectileFromCaster = attack_info.order {
                // do nothing, we did check_hit above
            } else {
                if check_hit(attack_info, attack_info.distance, hero.position) {
                    hero.receive_damage();
                }
            }
            let recover_info = RecoverInfo {
                started_at: Instant::now(),
                time_to_complete: 500,
            };
            self.recovering = Some(recover_info);
            match attack_info.kind {
                AttackKind::Wide => {
                    self.attack_index += 1;
                    if self.attack_index > 5 {
                        self.attack_index = 0;
                    }
                }
                AttackKind::Circle => {
                    self.ranged_attack_index += 1;
                    if self.ranged_attack_index > 1 {
                        self.ranged_attack_index = 0;
                    }
                }
                _ => (),
            }
            self.attacking = None;
        }
    }
    // if the boss attacks' are loaded from the external file
    // and look like Vec<AttackConstructor>
    // then it can be like
    // fn select_attack(
    //     attacks: Vec<AttackConstructor>,
    //     self_position: Position,
    //     player_position: Position,
    // ) -> AttackConstructor
    fn check_new_attack(&mut self, character_position: Point2<f32>) {
        if self.attacking.is_some() || self.recovering.is_some() {
            return;
        }
        let distance = distance_between(&self.position, &character_position);
        let mut direction = direction_from(&self.position, &character_position);
        if direction.norm() > 0.000_001 {
            direction.normalize_mut();
        }
        if distance < self.close_melee_attack_distance {
            // println!("Character is in CLOSE range, selecting target");
            let attack_info = match self.attack_index {
                0 => AttackInfo::wide(self.position, direction, self.close_melee_attack_distance),
                1 => AttackInfo::wide_right(
                    self.position,
                    direction,
                    self.close_melee_attack_distance,
                ),
                2 => AttackInfo::split(self.position, direction, self.close_melee_attack_distance),
                3 => {
                    AttackInfo::closing(self.position, direction, self.close_melee_attack_distance)
                }
                4 => AttackInfo::left_then_right(
                    self.position,
                    direction,
                    self.close_melee_attack_distance,
                ),
                5 => AttackInfo::right_then_left(
                    self.position,
                    direction,
                    self.close_melee_attack_distance,
                ),
                _ => panic!("Unexpected attack index"),
            };
            self.attacking = Some(attack_info);
            return;
        }
        if distance < self.melee_attack_distance {
            // println!("Character is in range, selecting target");
            let attack_info =
                AttackInfo::narrow(self.position, direction, self.melee_attack_distance);
            self.attacking = Some(attack_info);
            return;
        }
        if distance < self.ranged_attack_distance {
            let attack_info = match self.ranged_attack_index {
                0 => {
                    let direction = Vector2::new(-direction.x, -direction.y);
                    AttackInfo::fireball(self.position, direction, 20.0)
                }
                1 => AttackInfo::fireblast(character_position, direction, 70.0),
                _ => panic!("Unexpected ranged attack index"),
            };
            self.attacking = Some(attack_info);
        }
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
