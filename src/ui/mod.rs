pub mod character_sheet;
pub mod widgets;

use eframe::egui;

pub fn setup_ui_styles(ctx: &egui::Context) {
    let style = (*ctx.style()).clone();
    // Customize the style here
    ctx.set_style(style);
} 