/// Models for the D&D character
/// boilerplate code

pub mod character;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub race: String,
    // Add more character attributes as needed
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::new(),
            class: String::new(),
            level: 1,
            race: String::new(),
        }
    }
} 