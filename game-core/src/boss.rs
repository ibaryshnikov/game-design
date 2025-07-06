use std::collections::HashMap;

use nalgebra::{Point2, Vector2};

use network::server;
use shared::action::Action;
use shared::attack::{
    AttackConstructor, AttackDamageConstructor, AttackInfo, AttackOrder, AttackPartConstructor,
    AttackRange, AttackSequenceConstructor, AttackShapeConstructor, CircleConstructor,
    ComplexAttack, ComplexAttackConstructor, RecoverInfo,
};
use shared::character::Character;
use shared::check_hit;
use shared::npc::{NpcConstructor, load_attacks, load_complex_attacks};
use shared::position::{direction_from, distance_between};

use crate::hero::Hero;
use crate::scene;

pub struct Boss {
    pub position: Point2<f32>,
    pub size: f32,
    attacks: Vec<AttackConstructor>,
    pub attacking: Option<AttackInfo>,
    recovering: Option<RecoverInfo>,
    attacks_complex: Vec<ComplexAttackConstructor>,
    pub attacking_complex: Option<ComplexAttack>,
    pub action: Option<Action>,
    pub hp: i32,
    pub max_hp: i32,
    pub time_since_defeated: u128,
    pub respawn_time: u128,
}

impl Character for Boss {
    fn get_recovering_state(&mut self) -> &mut Option<RecoverInfo> {
        &mut self.recovering
    }
    fn clear_recovering_state(&mut self) {
        self.recovering = None;
    }
}

fn get_complex_attack_constructor() -> ComplexAttackConstructor {
    let damage_constructor = AttackDamageConstructor {
        value: 10,
        instances: 1,
        delay_between_instances: 0,
    };
    let circle_1 = CircleConstructor {
        time_to_complete: 1500,
        radius: 20.0,
    };
    let attack_part_1 = AttackPartConstructor {
        time_to_complete: 1500,
        shape: AttackShapeConstructor::Circle(circle_1),
        radius: 20.0,
        damage: Some(damage_constructor.clone()),
    };
    let circle_2 = CircleConstructor {
        time_to_complete: 1500,
        radius: 30.0,
    };
    let attack_part_2 = AttackPartConstructor {
        time_to_complete: 1500,
        shape: AttackShapeConstructor::Circle(circle_2),
        radius: 30.0,
        damage: Some(damage_constructor),
    };
    let sequence_1 = AttackSequenceConstructor {
        position_offset: Point2::new(30.0, 0.0),
        parts: vec![attack_part_1],
    };
    let sequence_2 = AttackSequenceConstructor {
        position_offset: Point2::new(-30.0, 0.0),
        parts: vec![attack_part_2],
    };
    ComplexAttackConstructor {
        range: AttackRange {
            from: 300.0,
            to: 500.0,
        },
        sequences: vec![sequence_1, sequence_2],
    }
}

impl Boss {
    pub fn new(position: Point2<f32>) -> Self {
        Boss {
            position,
            size: 30.0,
            attacks: Vec::new(),
            attacking: None,
            recovering: None,
            attacks_complex: Vec::new(),
            attacking_complex: None,
            action: None,
            hp: 300,
            max_hp: 300,
            time_since_defeated: 0,
            respawn_time: 10_000, // 10s
        }
    }
    pub fn from_constructor(position: Point2<f32>, constructor: NpcConstructor) -> Self {
        let NpcConstructor {
            respawn_time,
            attacks,
            ..
        } = constructor;
        let attacks = load_attacks(attacks);
        // let attacks_complex = load_complex_attacks(Vec::new());
        let attacks_complex = vec![get_complex_attack_constructor()];
        Boss {
            position,
            size: 30.0,
            attacks,
            attacking: None,
            recovering: None,
            attacks_complex,
            attacking_complex: None,
            action: None,
            hp: 300,
            max_hp: 300,
            time_since_defeated: 0,
            respawn_time: respawn_time * 1000, // change s to ms
        }
    }
    pub fn to_network(&self) -> server::Boss {
        server::Boss {
            position: self.position,
            attacking: self.attacking.clone(),
            recovering: self.recovering.clone(),
            attacking_complex: self.attacking_complex.clone(),
            action: self.action.clone(),
            hp: self.hp,
            max_hp: self.max_hp,
        }
    }
    pub fn from_network(boss: server::Boss) -> Self {
        Self {
            position: boss.position,
            size: 30.0,
            attacks: Vec::new(),
            attacking: boss.attacking.clone(),
            recovering: boss.recovering.clone(),
            attacks_complex: Vec::new(),
            attacking_complex: boss.attacking_complex.clone(),
            action: boss.action.clone(),
            hp: boss.hp,
            max_hp: boss.max_hp,
            time_since_defeated: 0,
            respawn_time: 0, // don't track respawn time on the client
        }
    }
    pub fn reset(&mut self) {
        // self.hp = self.max_hp;
    }
    pub fn stop(&mut self) {
        self.attacking = None;
        self.recovering = None;
    }
    pub fn update(
        &mut self,
        characters: &mut HashMap<u128, Hero>,
        dt: u128,
        scene_mode: scene::Mode,
    ) -> bool {
        self.update_attack(characters, dt);
        self.update_attack_complex(characters, dt);
        self.update_recovery(dt);

        if let scene::Mode::Client = scene_mode {
            return false;
        }

        self.check_respawn_time(dt);

        if characters.is_empty() {
            return false;
        }
        let index = if characters.len() == 1 {
            0
        } else {
            rand::random_range(0..characters.len())
        };
        if let Some(character) = characters.values().nth(index) {
            self.check_new_attack(character.position)
        } else {
            false
        }
    }
    fn update_attack(&mut self, characters: &mut HashMap<u128, Hero>, dt: u128) {
        let Some(attack_info) = &mut self.attacking else {
            return;
        };
        attack_info.update(dt);
        #[allow(clippy::single_match)] // will be extended later probably
        match attack_info.order {
            AttackOrder::ProjectileFromCaster => {
                if !attack_info.damage_done {
                    for hero in characters.values_mut() {
                        if check_hit(attack_info, attack_info.distance, hero.position, hero.size) {
                            hero.receive_damage();
                            attack_info.damage_done = true;
                        }
                    }
                }
            }
            _ => (),
        }
        if attack_info.completed() {
            if let AttackOrder::ProjectileFromCaster = attack_info.order {
                // do nothing, we did check_hit above
            } else {
                for hero in characters.values_mut() {
                    if check_hit(attack_info, attack_info.distance, hero.position, hero.size) {
                        hero.receive_damage();
                    }
                }
            }
            self.recovering = Some(RecoverInfo::new(attack_info.aftercast));
            self.attacking = None;
        }
    }
    fn update_attack_complex(&mut self, _characters: &mut HashMap<u128, Hero>, dt: u128) {
        let Some(attack_info) = &mut self.attacking_complex else {
            return;
        };
        attack_info.update(dt);
        if attack_info.completed() {
            self.attacking_complex = None;
        }
    }
    fn check_new_attack(&mut self, character_position: Point2<f32>) -> bool {
        if self.defeated() {
            return false;
        }
        if self.attacking.is_some() || self.recovering.is_some() {
            return false;
        }
        let distance = distance_between(&self.position, &character_position);
        let attacks: Vec<_> = self
            .attacks
            .iter()
            .filter(|attack| attack.range.in_range(distance))
            .collect();
        if attacks.is_empty() {
            return false;
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
                let range = constructor.range.to;
                AttackInfo::from_constructor(constructor, self.position, direction, range)
            }
        };
        self.attacking = Some(attack_info);
        true // send updates to client if it's a server
    }
    fn check_new_attack_complex(&mut self, character_position: Point2<f32>) {
        if self.defeated() {
            return;
        }
        if self.attacking_complex.is_some() {
            return;
        }
        let distance = distance_between(&self.position, &character_position);
        let attacks: Vec<_> = self
            .attacks_complex
            .iter()
            .filter(|attack| attack.range.in_range(distance))
            .collect();
        if attacks.is_empty() {
            return;
        }
        let index = rand::random_range(0..attacks.len());
        let constructor = attacks[index].clone();

        let dx = self.position.x - character_position.x;
        let dy = self.position.y - character_position.y;
        let direction_angle = dy.atan2(dx) + std::f32::consts::PI;

        let mut direction = direction_from(&self.position, &character_position);
        if direction.norm() > 0.000_001 {
            direction.normalize_mut();
        }
        let direction = Vector2::new(-direction.x, -direction.y);
        let attack = ComplexAttack::from_constructor(
            constructor,
            self.position,
            character_position,
            direction,
            direction_angle,
        );
        self.attacking_complex = Some(attack);
    }
    pub fn receive_damage(&mut self) {
        if self.defeated() {
            return;
        }
        self.hp -= 35;
        if self.hp < 0 {
            self.hp = 0;
        }
        if self.hp == 0 {
            self.time_since_defeated = 0;
        }
    }
    pub fn hp_left_percent(&self) -> f32 {
        self.hp as f32 / self.max_hp as f32
    }
    pub fn defeated(&self) -> bool {
        self.hp <= 0
    }
    fn check_respawn_time(&mut self, dt: u128) {
        if !self.defeated() {
            return;
        }
        self.time_since_defeated += dt;
        // println!("time since defeated: {}", self.time_since_defeated);
        if self.time_since_defeated > self.respawn_time {
            self.hp = self.max_hp;
        }
    }
}
