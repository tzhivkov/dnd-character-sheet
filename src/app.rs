use eframe::egui;
use dnd_character_creator::models::Character;
use dnd_character_creator::ui::character_sheet::CharacterSheet;

pub struct DndCharacterCreator {
    character: Character,
    character_sheet: CharacterSheet,
}

impl DndCharacterCreator {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            character: Character::default(),
            character_sheet: CharacterSheet::new(),
        }
    }
}

impl eframe::App for DndCharacterCreator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.character_sheet.show(ui);
        });
    }
} 