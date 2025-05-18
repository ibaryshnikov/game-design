use std::time::Instant;

use iced_core::{Color, Size};
use iced_widget::canvas::{stroke, Frame, Path, Stroke};
use nalgebra::Point2;

use shared::attack::{AttackInfo, AttackKind, RecoverInfo};
use shared::character::Character;
use shared::check_hit;
use shared::npc::NpcConstructor;
use shared::position::{direction_from, distance_between};

use crate::attack::AttackView;
use crate::hero::Hero;

pub struct Boss {
    pub position: Point2<f32>,
    close_melee_attack_distance: f32,
    attack_index: u8,
    pub melee_attack_distance: f32,
    pub ranged_attack_distance: f32,
    pub attacking: Option<AttackView>,
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
        let Some(attack_view) = &mut self.attacking else {
            return;
        };
        attack_view.update();
        if attack_view.completed() {
            if check_hit(
                &attack_view.attack_info,
                attack_view.attack_info.distance,
                hero.position,
            ) {
                hero.receive_damage();
            }
            let recover_info = RecoverInfo {
                started_at: Instant::now(),
                time_to_complete: 500,
            };
            self.recovering = Some(recover_info);
            if let AttackKind::Wide = attack_view.attack_info.kind {
                self.attack_index += 1;
                if self.attack_index > 5 {
                    self.attack_index = 0;
                }
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
            self.attacking = Some(AttackView::new(attack_info));
            return;
        }
        if distance < self.melee_attack_distance {
            // println!("Character is in range, selecting target");
            let attack_info =
                AttackInfo::narrow(self.position, direction, self.melee_attack_distance);
            self.attacking = Some(AttackView::new(attack_info));
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
    fn hp_left_percent(&self) -> f32 {
        self.hp as f32 / self.max_hp as f32
    }
    pub fn draw_body(&self, frame: &mut Frame) {
        let position = iced_core::Point::new(self.position.x, self.position.y);
        let path = Path::new(|b| {
            b.circle(position, 30.0);
        });
        frame.stroke(
            &path,
            Stroke {
                style: stroke::Style::Solid(Color::BLACK),
                width: 3.0,
                ..Stroke::default()
            },
        );
    }
    pub fn draw_health_bar(&self, frame: &mut Frame) {
        // self.draw_test_data(frame);
        let start = iced_core::Point::new(100.0, 700.0);
        let bar_width = 800.0;
        let bar_height = 10.0;

        // draw black background
        let path = Path::new(|b| {
            let size = Size::new(bar_width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(0, 0, 0));

        // draw hp left as green
        let path = Path::new(|b| {
            let width = bar_width * self.hp_left_percent();
            let size = Size::new(width, bar_height);
            b.rectangle(start, size);
        });

        frame.fill(&path, Color::from_rgb8(255, 0, 0));
    }
    pub fn draw_attack(&self, frame: &mut Frame) {
        if let Some(attack) = &self.attacking {
            attack.draw(frame);
        }
    }
}
