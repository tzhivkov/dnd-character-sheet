/// API module (change API_BASE_URL to the actual API URL when not testing locally)
mod client;
mod models;
mod cache;
use std::error::Error;

pub use client::DndApiClient;
pub use models::*;
pub use cache::Cache;

// Re-export commonly used types
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

// API endpoints
pub const API_BASE_URL: &str = "http://localhost:8000"; 