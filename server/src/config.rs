use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: u32,
}

impl Config {
    pub fn read_from_file() -> Self {
        let data = std::fs::read_to_string("./config.toml").expect("Should read server Config");
        toml::from_str(&data).expect("Should decode server Config")
    }
}
