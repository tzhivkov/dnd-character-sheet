/// Not implemented
/// 
/// This is going to be a simple cache for API responses. 
/// boilerplate code

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct Cache<T> {
    data: Mutex<HashMap<String, CacheEntry<T>>>,
    ttl: Duration,
}

struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

impl<T: Clone> Cache<T> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        if let Some(entry) = data.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            }
            data.remove(key);
        }
        None
    }

    pub fn set(&self, key: String, value: T) {
        let mut data = self.data.lock().unwrap();
        data.insert(
            key,
            CacheEntry {
                data: value,
                expires_at: Instant::now() + self.ttl,
            },
        );
    }
} 