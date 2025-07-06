pub mod constants;

pub fn save_character(_character: &crate::models::Character) -> Result<(), Box<dyn std::error::Error>> {
    // Implement character saving logic
    Ok(())
}

pub fn load_character() -> Result<crate::models::Character, Box<dyn std::error::Error>> {
    // Implement character loading logic
    unimplemented!()
} 