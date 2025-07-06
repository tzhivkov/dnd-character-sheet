/// A model for a D&D character
/// boilerplate code

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub race: String,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::new(),
            class: String::new(),
            level: 1,
            race: String::new(),
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
} 