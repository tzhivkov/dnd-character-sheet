/// A client for the open D&D API https://open5e.com/api-docs
use super::Result;
use reqwest::Client;
use std::time::Duration;
use super::models::{Spell, ApiResponse, Class, Race};

pub struct DndApiClient {
    client: Client,
    base_url: String,
}

impl Default for DndApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DndApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: super::API_BASE_URL.to_string(),
        }
    }

    // Helper method to convert various spell name formats to potential slugs
    fn generate_slug_variants(&self, input: &str) -> Vec<String> {
        let mut variants = Vec::new();
        
        // Convert to lowercase and remove extra whitespace
        let cleaned = input.trim().to_lowercase();
        
        // Only add the original input if it doesn't contain spaces (i.e., it's already a slug)
        if !cleaned.contains(' ') {
            variants.push(cleaned.clone());
        }
        
        // Convert spaces to hyphens
        let hyphenated = cleaned.replace(' ', "-");
        variants.push(hyphenated);
        
        // Convert spaces to underscores
        let underscored = cleaned.replace(' ', "_");
        variants.push(underscored);
        
        // Remove duplicates while preserving order
        variants.sort();
        variants.dedup();
        
        variants
    }

    // Try to find a spell by various name formats
    pub async fn find_spell_by_name(&self, spell_name: &str) -> Result<Option<Spell>> {
        let variants = self.generate_slug_variants(spell_name);
        
        println!("Trying to find spell '{}' with variants:", spell_name);
        for (i, variant) in variants.iter().enumerate() {
            print!("  {}. {}", i + 1, variant);
            
            match self.get_spell_by_slug(variant).await {
                Ok(spell) => {
                    println!(" ✅ Found!");
                    return Ok(Some(spell));
                }
                Err(e) => {
                    // Check if it's a 404 error (spell not found)
                    if e.to_string().contains("404") || e.to_string().contains("No Spell matches") {
                        println!(" ❌ Not found");
                    } else {
                        println!(" ❌ Error: {}", e);
                    }
                }
            }
        }
        
        println!("No spell found with any of the attempted variants.");
        Ok(None)
    }

    pub async fn get_classes(&self) -> Result<Vec<String>> {
        let url = format!("{}/v1/classes/", self.base_url);
        let response = self.client.get(&url).send().await?;
        let api_response: ApiResponse<Class> = response.json().await?;

        // Extract just the class names from the results
        let class_names: Vec<String> = api_response.results
            .into_iter()
            .filter_map(|class| class.name())
            .collect();
        
        Ok(class_names)
    }

    pub async fn get_races(&self) -> Result<Vec<String>> {
        let url = format!("{}/v1/races/", self.base_url);
        let response = self.client.get(&url).send().await?;
        let response_text = response.text().await?;
        let api_response: ApiResponse<Race> = serde_json::from_str(&response_text)?;
        
        // Extract just the race names from the results
        let race_names: Vec<String> = api_response.results
            .into_iter()
            .filter_map(|race| race.name())
            .collect();
        
        Ok(race_names)
    }

    pub async fn search_spells(&self, search_term: &str) -> Result<Vec<Spell>> {
        let url = format!("{}/spells/?search={}", self.base_url, search_term);
        let response = self.client.get(&url).send().await?;
        let response_text = response.text().await?;

        // Parse the response
        let api_response: ApiResponse<Spell> = serde_json::from_str(&response_text)?;
        
        // Filter spells to only include those with the search term in their slug
        let filtered_spells: Vec<Spell> = api_response.results
            .into_iter()
            .filter(|spell| {
                spell.slug()
                    .map(|slug| slug.contains(&search_term.to_lowercase()))
                    .unwrap_or(false)
            })
            .collect();
        
        Ok(filtered_spells)
    }

    pub async fn get_spell_by_slug(&self, slug: &str) -> Result<Spell> {
        let url = format!("{}/v1/spells/{}", self.base_url, slug);
        let response = self.client.get(&url).send().await?;
        
        // Check if the response is successful
        if !response.status().is_success() {
            let status = response.status();
            let response_text = response.text().await?;
            return Err(format!("HTTP {}: {}", status, response_text).into());
        }
        
        let spell: Spell = response.json().await?;
        Ok(spell)
    }

    // Add more API methods as needed
} 