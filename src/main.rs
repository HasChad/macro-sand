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

#[derive(PartialEq)]
struct Brush {
    brush_type: Cell,
    pos: Vec<Vec2>,
    size: usize,
}

impl Brush {
    fn new() -> Self {
        let size: usize = 5;
        let mut pos: Vec<Vec2> = vec![];

        for y in 0..size {
            for x in 0..size {
                pos.push(Vec2::new(x as f32, y as f32));
            }
        }

        Self {
            brush_type: Cell::sand(),
            pos,
            size,
        }
    }

    pub fn size_update(self: &mut Self, size: usize) {
        self.pos.clear();

        for y in 0..size {
            for x in 0..size {
                self.pos.push(Vec2::new(x as f32, y as f32));
            }
        }
    }
}

struct AppState {
    cells: Vec<Cell>,
    buffer: Vec<Cell>,
    brush: Brush,
    image: Image,
    texture: Texture2D,
    render_right: bool,
    can_draw: bool,
    tick_accumulator: f64,
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
            brush: Brush::new(),
            image: image.clone(),
            texture: Texture2D::from_image(&image),
            render_right: false,
            can_draw: true,
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

        app_state.tick_accumulator += get_frame_time() as f64;
        if app_state.tick_accumulator >= TICK_DURATION {
            update_world(&mut app_state).await;
            app_state.tick_accumulator -= TICK_DURATION;
        }

        if app_state.can_draw {
            update_brush(&mut app_state).await;
        }

        ui(&mut app_state);
        draw_world(&mut app_state).await;
        egui_macroquad::draw();

        next_frame().await
    }

    Ok(())
}
