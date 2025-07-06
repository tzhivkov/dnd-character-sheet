use dnd_character_creator::utils::constants::{WINDOW_WIDTH, WINDOW_HEIGHT};
use eframe::egui;

mod app;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };
    
    eframe::run_native(
        "DnD Character Creator",
        options,
        Box::new(|cc| Box::new(app::DndCharacterCreator::new(cc))),
    )
} 