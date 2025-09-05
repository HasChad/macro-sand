use egui_macroquad::egui;

use crate::{cells::Cell, AppState};

pub fn ui(state: &mut AppState) {
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Settings")
            .resizable(false)
            .collapsible(false)
            .show(egui_ctx, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Materials: ");
                        ui.end_row();
                        ui.radio_value(&mut state.brush, Cell::sand(), "Sand");
                        ui.end_row();
                        ui.radio_value(&mut state.brush, Cell::water(), "Water");
                        ui.end_row();
                        ui.radio_value(&mut state.brush, Cell::stone(), "Stone");
                        ui.end_row();
                    })
            });
    });
}
