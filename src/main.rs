#![windows_subsystem = "windows"]

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
const TICK_RATE: f64 = 120.0;
const TICK_DURATION: f64 = 1.0 / TICK_RATE;

enum Test {
    Ah,
    Uh,
}

pub struct AppState {
    cells: Vec<Cell>,
    buffer: Vec<Cell>,
    brush: Cell,
    image: Image,
    texture: Texture2D,
    render_right: bool,
    tick_accumulator: f64,
}

impl AppState {
    fn new() -> Self {
        let image = Image::gen_image_color(
            GRID_X_SIZE as u16,
            GRID_Y_SIZE as u16,
            Color::from_rgba(10, 10, 10, 255),
        );

        // testing comment
        Self {
            buffer: vec![Cell::empty(); GRID_X_SIZE * GRID_Y_SIZE],
            cells: vec![Cell::empty(); GRID_X_SIZE * GRID_Y_SIZE],
            brush: Cell::sand(),
            image: image.clone(),
            texture: Texture2D::from_image(&image),
            render_right: false,
            tick_accumulator: 0.0,
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

        println!("tesing");
        info!("test");

        app_state.tick_accumulator += get_frame_time() as f64;
        while app_state.tick_accumulator >= TICK_DURATION {
            update_brush(&mut app_state).await;
            update_world(&mut app_state).await;
            app_state.tick_accumulator -= TICK_DURATION;
        }

        ui(&mut app_state);
        draw_world(&mut app_state).await;
        egui_macroquad::draw();

        next_frame().await
    }
    Ok(())
}
