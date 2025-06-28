use std::collections::HashMap;
use std::time::Instant;

use nalgebra::Point2;

use game_core::boss::Boss;
use game_core::hero::Hero;
use game_core::scene::{self, Scene};
use shared::level::{Level, LevelList};
use shared::npc::NpcConstructor;

pub struct Stage {
    last_update: Instant,
    pub scene: Scene,
}

fn load_npc_by_id(id: u32) -> NpcConstructor {
    let file_path = format!("../data/npc/npc_{id}.json");
    let contents = std::fs::read(file_path).expect("Should read NpcConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode NpcConstructor")
}

fn load_level_list() -> LevelList {
    let file_path = "../data/level/list.json";
    let contents = std::fs::read(file_path).expect("Should read LevelList from a file");
    serde_json::from_slice(&contents).expect("Should decode LevelList")
}

fn load_level_by_id(id: u32) -> Level {
    let file_path = format!("../data/level/level_{id}.json");
    let contents = std::fs::read(file_path).expect("Should read Level from a file");
    serde_json::from_slice(&contents).expect("Should decode Level")
}

impl Stage {
    pub fn new() -> Self {
        let level_list = load_level_list();
        let boss_constructor = load_npc_by_id(1);
        let boss = Boss::from_constructor(Point2::new(512.0, 384.0), boss_constructor);
        let scene = Scene {
            frame_number: 0,
            mode: scene::Mode::Server,
            characters: HashMap::new(),
            npc: vec![boss],
            effects: Vec::new(),
            projectiles: Vec::new(),
        };
        Stage {
            last_update: Instant::now(),
            scene,
        }
    }
    fn load_level(&mut self, id: u32) {
        let level = load_level_by_id(id);
        let npc = &level.npc_list[0];
        let constructor = load_npc_by_id(npc.id);
        let boss = Boss::from_constructor(Point2::new(512.0, 384.0), constructor);
        self.scene.npc = vec![boss];
    }
    pub fn add_character(&mut self, id: u128, hero: Hero) {
        self.scene.characters.insert(id, hero);
    }

    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let dt = now.saturating_duration_since(self.last_update).as_millis();
        self.last_update = now;
        self.scene.update(dt)
    }
}
