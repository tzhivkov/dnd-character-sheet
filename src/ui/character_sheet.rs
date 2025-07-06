/// A character sheet for a D&D character
/// This will currently create a window with a heading and a button that does nothing
/// boilerplate code


use eframe::egui;

pub struct CharacterSheet {
    // Add character sheet state here
}

impl Default for CharacterSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterSheet {
    pub fn new() -> Self {
        Self {
            // Initialize state
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Character Sheet");
        // Add character sheet UI components here
    }
} 