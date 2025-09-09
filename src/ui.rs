use egui_macroquad::egui::{self, Slider};

use crate::{cells::Cell, AppState};

pub fn ui(state: &mut AppState) {
    egui_macroquad::ui(|ctx| {
        state.can_draw = !ctx.wants_pointer_input();

        egui::Window::new("Settings")
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Materials: ");
                        ui.end_row();
                        ui.radio_value(&mut state.brush.brush_type, Cell::sand(), "Sand");
                        ui.end_row();
                        ui.radio_value(&mut state.brush.brush_type, Cell::water(), "Water");
                        ui.end_row();
                        ui.radio_value(&mut state.brush.brush_type, Cell::stone(), "Stone");
                        ui.end_row();
                        if ui.add(Slider::new(&mut state.brush.size, 1..=10)).changed() {
                            state.brush.size_update(state.brush.size);
                        }
                    })
            });
    });
}
