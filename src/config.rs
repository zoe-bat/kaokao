use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub rofi_binary: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rofi_binary: "rofi".into(),
        }
    }
}

pub fn load_config() -> Config {
    confy::load("kaokao", None).unwrap_or(Config::default())
}
