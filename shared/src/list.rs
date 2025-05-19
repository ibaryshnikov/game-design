use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum EntryStatus {
    Active,
    Hidden,
}

impl EntryStatus {
    pub fn is_active(&self) -> bool {
        match self {
            EntryStatus::Active => true,
            EntryStatus::Hidden => false,
        }
    }
}
