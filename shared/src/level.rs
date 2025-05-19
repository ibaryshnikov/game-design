use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::list::EntryStatus;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LevelList {
    pub last_id: u32,
    pub list: Vec<LevelInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LevelInfo {
    pub id: u32,
    pub name: String,
    pub status: EntryStatus,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct LevelNpcInfo {
    pub id: u32,
    pub name: String,
}

impl Display for LevelNpcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Level {
    pub name: String,
    pub npc_list: Vec<LevelNpcInfo>,
}

impl Level {
    pub fn new(name: String) -> Self {
        Self {
            name,
            npc_list: Vec::new(),
        }
    }
}
