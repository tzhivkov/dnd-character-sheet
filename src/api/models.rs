/// Models for the D&D API
/// 
/// This is a simple model for the D&D API. It is used to parse the API responses.
/// 
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

// Generic dictionary type for any open5e.com API response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnDict {
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

impl DnDict {
    // Helper method to get a string field
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.fields.get(key)?.as_str().map(|s| s.to_string())
    }
    
    // Helper method to get an integer field
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.fields.get(key)?.as_i64().map(|i| i as i32)
    }
    
    // Helper method to get a boolean field
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.fields.get(key)?.as_bool()
    }
    
    // Helper method to get a vector of strings
    pub fn get_string_vec(&self, key: &str) -> Option<Vec<String>> {
        self.fields.get(key)?.as_array()?.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect::<Vec<String>>()
            .into()
    }
    
    // Helper method to get a nested DnDict
    pub fn get_dict(&self, key: &str) -> Option<DnDict> {
        let value = self.fields.get(key)?;
        serde_json::from_value(value.clone()).ok()
    }
    
    // Helper method to get a vector of DnDict
    pub fn get_dict_vec(&self, key: &str) -> Option<Vec<DnDict>> {
        let value = self.fields.get(key)?;
        serde_json::from_value(value.clone()).ok()
    }
    
    // Get the name field (common across many D&D entities)
    pub fn name(&self) -> Option<String> {
        self.get_string("name")
    }
    
    // Get the slug field (common across many D&D entities)
    pub fn slug(&self) -> Option<String> {
        self.get_string("slug")
    }
}

// Type aliases for convenience (is this overkill?)
pub type Class = DnDict;
pub type Race = DnDict;
pub type Spell = DnDict;
pub type School = DnDict;
pub type ClassReference = DnDict; 