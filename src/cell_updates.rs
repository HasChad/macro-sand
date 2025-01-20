use crate::{
    cells::{Cell, CellState, Direction},
    GRID_X_SIZE, GRID_Y_SIZE,
};

pub async fn update_world(cells: &mut [Cell], buffer: &mut [Cell]) {
    // Pixel iterate
    for y in (0..GRID_Y_SIZE).rev() {
        for x in 0..GRID_X_SIZE {
            let pixel_pos: usize = (y * GRID_X_SIZE) + x;

            match cells[pixel_pos].state {
                CellState::Sand => update_sand(x, y, pixel_pos, cells, buffer).await,
                CellState::Water => update_water(x, y, pixel_pos, cells, buffer).await,
                _ => (),
            }
        }
    }
}

pub async fn update_sand(
    x: usize,
    y: usize,
    pixel_pos: usize,
    cells: &mut [Cell],
    buffer: &mut [Cell],
) {
    if y == GRID_Y_SIZE - 1 {
        return;
    }

    let down: usize = pixel_pos + GRID_X_SIZE;
    let down_left: usize = down - 1;
    let down_right: usize = down + 1;

    // Down-Side checker
    let downleft_is_empty = cells[down_left] == Cell::spawn_empty() && x != 0;
    let downright_is_empty = cells[down_right] == Cell::spawn_empty() && x != GRID_X_SIZE - 1;

    // Down
    if cells[down] == Cell::spawn_empty() {
        buffer[down] = Cell::spawn_sand();
        buffer[pixel_pos] = Cell::spawn_empty();

    // Down water
    } else if cells[down].state == CellState::Water {
        buffer[down] = Cell::spawn_sand();
        buffer[pixel_pos] = Cell::spawn_water();

    // Down left
    } else if downleft_is_empty {
        buffer[down_left] = Cell::spawn_sand();
        buffer[pixel_pos] = Cell::spawn_empty();

    // Down right
    } else if downright_is_empty {
        buffer[down_right] = Cell::spawn_sand();
        buffer[pixel_pos] = Cell::spawn_empty();
    }
}

pub async fn update_water(
    x: usize,
    y: usize,
    pixel_pos: usize,
    cells: &mut [Cell],
    buffer: &mut [Cell],
) {
    let down: usize = pixel_pos + GRID_X_SIZE;
    let down_left: usize = down - 1;
    let down_right: usize = down + 1;
    let left: usize = pixel_pos - 1;
    let right: usize = pixel_pos + 1;

    // checkers
    let downleft_is_empty = cells[down_left] == Cell::spawn_empty();
    let downright_is_empty = cells[down_right] == Cell::spawn_empty();
    let left_is_empty = cells[left] == Cell::spawn_empty() && x != 0;
    let right_is_empty = cells[right] == Cell::spawn_empty() && x != GRID_X_SIZE - 1;

    //Down
    if cells[down] == Cell::spawn_empty() && y != GRID_Y_SIZE - 1 {
        buffer[down] = Cell::spawn_water();
        buffer[pixel_pos] = Cell::spawn_empty();

    // Down left
    } else if downleft_is_empty {
        buffer[down_left] = Cell::spawn_water();
        buffer[pixel_pos] = Cell::spawn_empty();

    // Down right
    } else if downright_is_empty {
        buffer[down_right] = Cell::spawn_water();
        buffer[pixel_pos] = Cell::spawn_empty();

    // Sides
    } else if left_is_empty || right_is_empty {
        if left_is_empty && right_is_empty {
            match cells[pixel_pos].move_direction {
                Direction::Right => {
                    buffer[right] = Cell::spawn_water();
                    buffer[pixel_pos] = Cell::spawn_empty();
                }
                Direction::Left => {
                    buffer[left] = Cell::spawn_water();
                    buffer[pixel_pos] = Cell::spawn_empty();
                }
                Direction::None => (),
            }
        //Right
        } else if right_is_empty {
            buffer[right] = Cell::spawn_water();
            buffer[pixel_pos] = Cell::spawn_empty();

        //Left
        } else if left_is_empty {
            buffer[left] = Cell::spawn_water();
            buffer[pixel_pos] = Cell::spawn_empty();
        }
    } /*
      else {
          match cells[pixel_pos].move_direction {
              Direction::Left => cells[pixel_pos].move_direction = Direction::Right,
              Direction::Right => cells[pixel_pos].move_direction = Direction::Left,
              Direction::None => (),
          }
      }
      */
}
