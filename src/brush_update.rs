use macroquad::prelude::*;

use crate::{cells::Cell, DOT_SIZE_IN_PXS, GRID_X_SIZE};

pub async fn update_brush(cells: &mut [Cell], brush: &mut Cell) {
    //Change Brush
    if let Some(input) = get_last_key_pressed() {
        match input {
            KeyCode::Key1 => *brush = Cell::spawn_sand(),
            KeyCode::Key2 => *brush = Cell::spawn_water(),
            KeyCode::Key3 => *brush = Cell::spawn_stone(),
            KeyCode::Space => {
                for cell in cells.iter_mut() {
                    *cell = Cell::spawn_empty();
                }
            }
            _ => (),
        }
    }

    //Mouse Click Spawn
    let (mouse_xpos, mouse_ypos) = mouse_position();
    let pixel_posx = mouse_xpos as usize / DOT_SIZE_IN_PXS;
    let pixel_posy = mouse_ypos as usize / DOT_SIZE_IN_PXS;
    let pixel_pos = pixel_posx + (pixel_posy * GRID_X_SIZE);

    if mouse_xpos >= 0.
        && mouse_xpos < screen_width()
        && mouse_ypos >= 0.
        && mouse_ypos < screen_height()
    {
        if is_mouse_button_down(MouseButton::Left) && cells[pixel_pos] == Cell::spawn_empty() {
            cells[pixel_pos] = *brush;

            //top
            // cells[pixel_pos - 2 * GRID_X_SIZE - 2] = *brush;
            // cells[pixel_pos - 2 * GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos - 2 * GRID_X_SIZE] = *brush;
            // cells[pixel_pos - 2 * GRID_X_SIZE + 1] = *brush;
            // cells[pixel_pos - 2 * GRID_X_SIZE + 2] = *brush;

            // cells[pixel_pos - GRID_X_SIZE - 2] = *brush;
            cells[pixel_pos - GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos - GRID_X_SIZE] = *brush;
            cells[pixel_pos - GRID_X_SIZE + 1] = *brush;
            // cells[pixel_pos - GRID_X_SIZE + 2] = *brush;

            //middle
            cells[pixel_pos - 2] = *brush;
            cells[pixel_pos - 1] = *brush;
            cells[pixel_pos + 1] = *brush;
            cells[pixel_pos + 2] = *brush;

            //bottom
            // cells[pixel_pos + GRID_X_SIZE - 2] = *brush;
            cells[pixel_pos + GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos + GRID_X_SIZE] = *brush;
            cells[pixel_pos + GRID_X_SIZE + 1] = *brush;
            // cells[pixel_pos + GRID_X_SIZE + 2] = *brush;

            // cells[pixel_pos + 2 * GRID_X_SIZE - 2] = *brush;
            // cells[pixel_pos + 2 * GRID_X_SIZE - 1] = *brush;
            cells[pixel_pos + 2 * GRID_X_SIZE] = *brush;
            // cells[pixel_pos + 2 * GRID_X_SIZE + 1] = *brush;
            // cells[pixel_pos + 2 * GRID_X_SIZE + 2] = *brush;
        }

        if is_mouse_button_down(MouseButton::Right) {
            cells[pixel_pos] = Cell::spawn_empty();

            //top
            cells[pixel_pos - GRID_X_SIZE - 1] = Cell::spawn_empty();
            cells[pixel_pos - GRID_X_SIZE] = Cell::spawn_empty();
            cells[pixel_pos - GRID_X_SIZE + 1] = Cell::spawn_empty();

            //middle
            cells[pixel_pos - 2] = Cell::spawn_empty();
            cells[pixel_pos - 1] = Cell::spawn_empty();
            cells[pixel_pos + 1] = Cell::spawn_empty();
            cells[pixel_pos + 2] = Cell::spawn_empty();

            //bottom
            cells[pixel_pos + GRID_X_SIZE - 1] = Cell::spawn_empty();
            cells[pixel_pos + GRID_X_SIZE] = Cell::spawn_empty();
            cells[pixel_pos + GRID_X_SIZE + 1] = Cell::spawn_empty();
        }
    }
}
