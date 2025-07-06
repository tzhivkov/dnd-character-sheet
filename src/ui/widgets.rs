use eframe::egui;

pub struct StatBox {
    label: String,
    value: i32,
}

impl StatBox {
    pub fn new(label: &str, value: i32) -> Self {
        Self {
            label: label.to_string(),
            value,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(&self.label);
            ui.label(format!("{}", self.value));
        });
    }
} 