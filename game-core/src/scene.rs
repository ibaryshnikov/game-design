use std::collections::HashMap;

use nalgebra::Point2;

use network::client;
use network::server;
use shared::effect::area;
use shared::projectile::Projectile;

use crate::boss::Boss;
use crate::hero::Hero;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Client,
    Server,
}

pub struct Scene {
    pub mode: Mode,
    pub characters: HashMap<u128, Hero>,
    pub npc: Vec<Boss>,
    pub effects: Vec<area::Effect>,
    pub projectiles: Vec<Projectile>,
}

impl Scene {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            characters: HashMap::new(),
            npc: Vec::new(),
            effects: Vec::new(),
            projectiles: Vec::new(),
        }
    }
    pub fn add_character(&mut self, id: u128) {
        let hero = Hero::new(id, Point2::new(250.0, 200.0));
        self.characters.insert(id, hero);
    }
    pub fn remove_character(&mut self, id: u128) {
        self.characters.remove(&id);
    }
    pub fn to_network(&self) -> server::Scene {
        let mut characters = HashMap::new();
        for (key, value) in self.characters.iter() {
            characters.insert(*key, value.to_network());
        }
        let npc = self.npc.iter().map(|item| item.to_network()).collect();
        server::Scene { characters, npc }
    }
    pub fn update_from_network(&mut self, scene: server::Scene) {
        for (key, network_character) in scene.characters.into_iter() {
            println!("Network character {:?}", network_character);
            let character = Hero::from_network(network_character);
            self.characters.insert(key, character);
        }
        // update npc as well
    }
    pub fn update(&mut self, dt: u128) -> bool {
        for hero in self.characters.values_mut() {
            hero.update(&mut self.npc, dt);
        }
        let mut update_event = false;
        for boss in self.npc.iter_mut() {
            if boss.update(&mut self.characters, dt, self.mode) {
                update_event = true;
            }
        }
        for effect in self.effects.iter_mut() {
            effect.update(dt);
        }
        for projectile in self.projectiles.iter_mut() {
            projectile.update(dt);
        }
        update_event
    }
    pub fn handle_server_message(&mut self, message: server::Message) {
        use server::Message;
        match message {
            Message::Test => {
                println!("Test message in game-core");
            }
            Message::SetId(_id) => {
                // do nothing here
            }
            Message::Update(update) => {
                self.handle_server_update(update);
            }
        }
    }
    pub fn handle_server_update(&mut self, update: server::Update) {
        use server::Update;
        match update {
            Update::Scene(scene) => {
                println!("game-core Scene update: {:?}", scene);
            }
            Update::Character(character_update) => {
                println!("game-core Character update: {:?}", character_update);
            }
            Update::NpcList(npc_list) => {
                println!("game-core NpcList update: {:?}", npc_list);
            }
            Update::Projectile => {
                // got projectile update
            }
            Update::Entity => {
                // got entity update
            }
        }
    }
    pub fn handle_client_message(&mut self, id: u128, message: client::Message) {
        let Some(hero) = self.characters.get_mut(&id) else {
            return;
        };
        use client::Message;

        match message {
            Message::Join => {
                println!("Message::Join in game-core scene");
            }
            Message::Move(kind, movement) => {
                println!(
                    "Message::Move in game-core scene: {:?} {:?}",
                    kind, movement
                );

                hero.handle_move_action(kind, movement);
            }
            Message::HeroDash => {
                println!("Message::HeroDash in game-core scene");
            }
            Message::HeroAttack => {
                println!("Message::HeroAttack in game-core scene");
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
