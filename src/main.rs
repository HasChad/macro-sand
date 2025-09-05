#![windows_subsystem = "windows"]

// use egui_macroquad::prelude::*;
use macroquad::prelude::*;

mod app_settings;
mod brush_update;
mod cell_updates;
mod cells;
mod draw;
mod ui;

use app_settings::*;
use brush_update::*;
use cell_updates::*;
use cells::*;
use draw::*;
use ui::*;

const GRID_X_SIZE: usize = 300;
const GRID_Y_SIZE: usize = 160;
const DOT_SIZE_IN_PXS: usize = 4;

pub struct AppState {
    cells: Vec<Cell>,
    buffer: Vec<Cell>,
    brush: Cell,
    image: Image,
    texture: Texture2D,
    render_right: bool,
}

impl AppState {
    fn new() -> Self {
        let image = Image::gen_image_color(
            GRID_X_SIZE as u16,
            GRID_Y_SIZE as u16,
            Color::from_rgba(10, 10, 10, 255),
        );

        Self {
            buffer: vec![Cell::empty(); GRID_X_SIZE * GRID_Y_SIZE],
            cells: vec![Cell::empty(); GRID_X_SIZE * GRID_Y_SIZE],
            brush: Cell::sand(),
            image: image.clone(),
            texture: Texture2D::from_image(&image),
            render_right: false,
        }
    }
}

#[macroquad::main(window_conf)]
pub async fn main() -> Result<(), String> {
    let mut app_state = AppState::new();

    'running: loop {
        if is_key_pressed(KeyCode::Escape) {
            break 'running;
        }

        // fps limiter
        let minimum_frame_time = 1. / 60.;
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        ui(&mut app_state);

        update_brush(&mut app_state).await;
        update_world(&mut app_state).await;
        draw_world(&mut app_state).await;

        egui_macroquad::draw();

        next_frame().await
    }

    Ok(())
}
