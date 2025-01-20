// #![windows_subsystem = "windows"]

use macroquad::prelude::*;

mod brush_update;
mod cell_updates;
mod cells;

use brush_update::*;
use cell_updates::*;
use cells::Cell;

const GRID_X_SIZE: usize = 300;
const GRID_Y_SIZE: usize = 160;
const DOT_SIZE_IN_PXS: usize = 4;

fn window_conf() -> Conf {
    Conf {
        window_title: "Macro-Sand".into(),
        window_width: (GRID_X_SIZE * DOT_SIZE_IN_PXS) as i32,
        window_height: (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() -> Result<(), String> {
    // Rendering things
    let mut image = Image::gen_image_color(
        GRID_X_SIZE as u16,
        GRID_Y_SIZE as u16,
        Color::from_rgba(10, 10, 10, 255),
    );
    let texture = Texture2D::from_image(&image);

    // Game things
    let mut buffer = vec![Cell::spawn_empty(); GRID_X_SIZE * GRID_Y_SIZE];
    let mut cells = vec![Cell::spawn_empty(); GRID_X_SIZE * GRID_Y_SIZE];
    let mut brush = Cell::spawn_sand();

    // Game Loop
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

        update_brush(&mut cells, &mut brush).await;
        update_world(&mut cells).await;
        draw_world(&mut cells, &mut image, &texture).await;

        next_frame().await
    }

    Ok(())
}

async fn draw_world(cells: &mut [Cell], image: &mut Image, texture: &Texture2D) {
    // Per-pixel coloring
    for (i, cell) in cells.iter_mut().enumerate() {
        image.set_pixel(
            (i % GRID_X_SIZE) as u32,
            (i / GRID_X_SIZE) as u32,
            cell.color,
        );
    }

    // Cursor
    let (mouse_xpos, mouse_ypos) = mouse_position();
    let pixel_posx = mouse_xpos as u32 / DOT_SIZE_IN_PXS as u32;
    let pixel_posy = mouse_ypos as u32 / DOT_SIZE_IN_PXS as u32;

    if mouse_xpos >= 0.
        && mouse_xpos < screen_width()
        && mouse_ypos >= 0.
        && mouse_ypos < screen_height()
    {
        // left
        if mouse_xpos > DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx - 1, pixel_posy, WHITE);
        }
        // right
        if mouse_xpos < screen_width() - DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx + 1, pixel_posy, WHITE);
        }
        // top
        if mouse_ypos > DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx, pixel_posy - 1, WHITE);
        }
        // bottom
        if mouse_ypos < screen_height() - DOT_SIZE_IN_PXS as f32 {
            image.set_pixel(pixel_posx, pixel_posy + 1, WHITE);
        }
    }

    // Rendering
    texture.update(image);
    texture.set_filter(FilterMode::Nearest);

    draw_texture_ex(
        texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 {
                x: screen_width(),
                y: screen_height(),
            }),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}
