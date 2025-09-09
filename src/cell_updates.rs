use crate::{
    cells::{Cell, CellMatter, CellType, Direction},
    AppState, GRID_X_SIZE, GRID_Y_SIZE,
};

pub async fn update_world(state: &mut AppState) {
    // Pixel iterate
    for y in (0..GRID_Y_SIZE).rev() {
        if state.render_right {
            for x in 0..GRID_X_SIZE {
                let pixel_pos: usize = x + (y * GRID_X_SIZE);

                if state.cells[pixel_pos].move_direction != Direction::None {
                    match state.cells[pixel_pos].matter {
                        CellMatter::Solid => {
                            update_solid(x, y, pixel_pos, &mut state.buffer, state.cells[pixel_pos])
                                .await
                        }
                        CellMatter::Liquid => {
                            update_liquid(x, y, pixel_pos, &mut state.buffer).await
                        }
                        _ => (),
                    }
                }
            }

            state.render_right = false
        } else {
            for x in (0..GRID_X_SIZE).rev() {
                let pixel_pos: usize = x + (y * GRID_X_SIZE);

                if state.cells[pixel_pos].move_direction != Direction::None {
                    match state.cells[pixel_pos].matter {
                        CellMatter::Solid => {
                            update_solid(x, y, pixel_pos, &mut state.buffer, state.cells[pixel_pos])
                                .await
                        }
                        CellMatter::Liquid => {
                            update_liquid(x, y, pixel_pos, &mut state.buffer).await
                        }
                        _ => (),
                    }
                }
            }

            state.render_right = true
        }
    }
}

pub async fn update_solid(
    x: usize,
    y: usize,
    pixel_pos: usize,
    buffer: &mut [Cell],
    cell_type: Cell,
) {
    if y == GRID_Y_SIZE - 1 {
        return;
    }

    let down: usize = pixel_pos + GRID_X_SIZE;
    let down_left: usize = down - 1;
    let down_right: usize = down + 1;

    // Checkers
    let down_is_empty = buffer[down] == Cell::empty();
    let downleft_is_empty = buffer[down_left] == Cell::empty() && x != 0;
    let downright_is_empty = if x != GRID_X_SIZE - 1 {
        buffer[down_right] == Cell::empty()
    } else {
        false
    };

    // Down
    if down_is_empty {
        buffer[pixel_pos] = Cell::empty();
        buffer[down] = cell_type;

    // Down water
    } else if buffer[down].cell_type == CellType::Water {
        buffer[pixel_pos] = Cell::water();
        buffer[down] = cell_type;

    // Down left
    } else if downleft_is_empty && buffer[pixel_pos].move_direction == Direction::Left {
        buffer[pixel_pos] = Cell::empty();
        buffer[down_left] = cell_type;
        buffer[down_left].move_direction = Direction::Left;

    // Down right
    } else if downright_is_empty && buffer[pixel_pos].move_direction == Direction::Right {
        buffer[pixel_pos] = Cell::empty();
        buffer[down_right] = cell_type;
        buffer[down_right].move_direction = Direction::Right;
    } else {
        match buffer[pixel_pos].move_direction {
            Direction::Right => buffer[pixel_pos].move_direction = Direction::Left,
            Direction::Left => buffer[pixel_pos].move_direction = Direction::Right,
            _ => (),
        }
    }
}

pub async fn update_liquid(x: usize, y: usize, pixel_pos: usize, buffer: &mut [Cell]) {
    // if y < 0
    // if downleft and right not available

    let mut go_side = false;

    if y != GRID_Y_SIZE - 1 {
        let down: usize = pixel_pos + GRID_X_SIZE;
        let down_left: usize = down - 1;
        let down_right: usize = down + 1;

        let downleft_is_empty = buffer[down_left] == Cell::empty() && x != 0;
        let downright_is_empty = if x != GRID_X_SIZE - 1 {
            buffer[down_right] == Cell::empty()
        } else {
            false
        };

        // Down
        if buffer[down] == Cell::empty() {
            buffer[pixel_pos] = Cell::empty();
            buffer[down] = Cell::water();

        // Down left
        } else if downleft_is_empty && buffer[pixel_pos].move_direction == Direction::Left {
            buffer[pixel_pos] = Cell::empty();
            buffer[down_left] = Cell::water();
            buffer[down_left].move_direction = Direction::Left;
        // Down right
        } else if downright_is_empty && buffer[pixel_pos].move_direction == Direction::Right {
            buffer[pixel_pos] = Cell::empty();
            buffer[down_right] = Cell::water();
            buffer[down_right].move_direction = Direction::Right;
        } else {
            go_side = true;
        }
    } else {
        go_side = true;
    }

    if go_side {
        let left: usize = pixel_pos - 1;
        let right: usize = pixel_pos + 1;

        let left_is_empty = buffer[left] == Cell::empty() && x != 0;
        let right_is_empty = if x != GRID_X_SIZE - 1 {
            buffer[right] == Cell::empty()
        } else {
            false
        };

        // Left
        if left_is_empty && buffer[pixel_pos].move_direction == Direction::Left {
            buffer[pixel_pos] = Cell::empty();
            buffer[left] = Cell::water();
            buffer[left].move_direction = Direction::Left;
        // Right
        } else if right_is_empty && buffer[pixel_pos].move_direction == Direction::Right {
            buffer[pixel_pos] = Cell::empty();
            buffer[right] = Cell::water();
            buffer[right].move_direction = Direction::Right;
        } else {
            match buffer[pixel_pos].move_direction {
                Direction::Right => buffer[pixel_pos].move_direction = Direction::Left,
                Direction::Left => buffer[pixel_pos].move_direction = Direction::Right,
                _ => (),
            };
        }
    }
}
