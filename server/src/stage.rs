use std::collections::HashMap;

use crate::hero::Hero;
// use crate::npc::Npc;
use crate::boss::Boss;

pub struct Stage {
    characters: HashMap<u128, Hero>,
    npc: Vec<Boss>,
}

impl Stage {
    pub fn new() -> Self {
        Stage {
            characters: HashMap::new(),
            npc: vec![],
        }
    }
    pub fn add_character(&mut self, id: u128, hero: Hero) {
        self.characters.insert(id, hero);
    }

    pub fn update(&mut self) {
        for (_, character) in self.characters.iter_mut() {
            character.update(&mut self.npc);
        }
    }
}
