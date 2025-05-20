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

    // Checkers
    let down_is_empty = cells[down] == Cell::spawn_empty() && buffer[down] == Cell::spawn_empty();
    let downleft_is_empty = cells[down_left] == Cell::spawn_empty()
        && buffer[down_left] == Cell::spawn_empty()
        && x != 0;
    let downright_is_empty = cells[down_right] == Cell::spawn_empty()
        && buffer[down_right] == Cell::spawn_empty()
        && x != GRID_X_SIZE - 1;

    // Down
    if down_is_empty {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[down] = Cell::spawn_sand();

    // Down water
    } else if cells[down].state == CellState::Water {
        buffer[pixel_pos] = Cell::spawn_water();
        buffer[down] = Cell::spawn_sand();

    // Down left
    } else if downleft_is_empty {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[down_left] = Cell::spawn_sand();

    // Down right
    } else if downright_is_empty {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[down_right] = Cell::spawn_sand();
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

    // Checkers
    let downleft_is_empty = cells[down_left] == Cell::spawn_empty() && x != 0;
    let downright_is_empty = cells[down_right] == Cell::spawn_empty() && x != GRID_X_SIZE - 1;
    let left_is_empty =
        cells[left] == Cell::spawn_empty() && buffer[left] == Cell::spawn_empty() && x != 0;
    let right_is_empty = cells[right] == Cell::spawn_empty()
        && buffer[right] == Cell::spawn_empty()
        && x != GRID_X_SIZE - 1;

    // Down
    if cells[down] == Cell::spawn_empty() && y != GRID_Y_SIZE - 1 {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[down] = Cell::spawn_water();

    // Down left
    } else if downleft_is_empty {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[down_left] = Cell::spawn_water();

    // Down right
    } else if downright_is_empty {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[down_right] = Cell::spawn_water();

    // Sides
    } else if left_is_empty && cells[pixel_pos].move_direction == Direction::Left {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[left] = Cell::spawn_water();
        buffer[left].move_direction = Direction::Left;
    // Right
    } else if right_is_empty && cells[pixel_pos].move_direction == Direction::Right {
        buffer[pixel_pos] = Cell::spawn_empty();
        buffer[right] = Cell::spawn_water();
        buffer[right].move_direction = Direction::Right;
    } else {
        match cells[pixel_pos].move_direction {
            Direction::Right => buffer[pixel_pos].move_direction = Direction::Left,
            Direction::Left => buffer[pixel_pos].move_direction = Direction::Right,
            Direction::None => todo!(),
        };
    }
}

/*
if left_is_empty && right_is_empty {
            match cells[pixel_pos].move_direction {
                Direction::Right => {
                    buffer[pixel_pos] = Cell::spawn_empty();
                    buffer[right] = Cell::spawn_water();
                }
                Direction::Left => {
                    buffer[pixel_pos] = Cell::spawn_empty();
                    buffer[left] = Cell::spawn_water();
                }
                Direction::None => (),
            }
        // Left
        } else
*/
