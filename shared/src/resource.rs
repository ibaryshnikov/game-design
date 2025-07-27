use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceConstructor {
    pub name: String,
}

impl ResourceConstructor {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct ResourceInfo {
    pub name: String,
}
