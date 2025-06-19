use std::collections::HashMap;

use shared::effect::area;
use shared::projectile::Projectile;

use crate::boss::Boss;
use crate::hero::Hero;
use crate::server::ServerMessage;

pub struct Scene {
    pub characters: HashMap<u128, Hero>,
    pub npc: Vec<Boss>,
    pub effects: Vec<area::Effect>,
    pub projectiles: Vec<Projectile>,
}

impl Scene {
    pub fn new(hero: Hero, boss: Boss) -> Self {
        let mut characters = HashMap::new();
        characters.insert(0, hero);
        Self {
            characters,
            npc: vec![boss],
            effects: Vec::new(),
            projectiles: Vec::new(),
        }
    }
    pub fn update(&mut self, dt: u128) {
        for hero in self.characters.values_mut() {
            hero.update(&mut self.npc, dt);
        }
        for boss in self.npc.iter_mut() {
            boss.update(&mut self.characters, dt);
        }
        for effect in self.effects.iter_mut() {
            effect.update(dt);
        }
        for projectile in self.projectiles.iter_mut() {
            projectile.update(dt);
        }
    }
    pub fn handle_server_message(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::Test => {
                println!("Test message in game-core");
            }
            ServerMessage::Scene(scene) => {
                self.characters = scene.characters;
                self.npc = scene.npc;
            }
        }
    }
    pub fn stop(&mut self) {
        for hero in self.characters.values_mut() {
            hero.stop();
        }
        for boss in self.npc.iter_mut() {
            boss.stop();
        }
    }
    pub fn reset(&mut self) {
        for hero in self.characters.values_mut() {
            hero.reset();
        }
        for boss in self.npc.iter_mut() {
            boss.reset();
        }
        self.effects = Vec::new();
        self.projectiles = Vec::new();
    }
}
