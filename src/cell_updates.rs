use crate::{
    cells::{Cell, CellState, Direction},
    GRID_X_SIZE, GRID_Y_SIZE,
};

pub async fn update_sand(x: usize, y: usize, pixel_pos: usize, cells: &mut [Cell]) {
    let down: usize = if y < GRID_Y_SIZE - 1 {
        pixel_pos + GRID_X_SIZE
    } else {
        pixel_pos
    };

    let down_left: usize = if x > 0 && down != pixel_pos {
        down - 1
    } else {
        pixel_pos
    };

    let down_right: usize = if x < GRID_X_SIZE - 1 && down != pixel_pos {
        down + 1
    } else {
        pixel_pos
    };

    // Down-Side checker
    let downleft_is_empty = cells[down_left] == Cell::spawn_empty();
    let downright_is_empty = cells[down_right] == Cell::spawn_empty();

    // Down
    if cells[down] == Cell::spawn_empty() {
        cells[down] = Cell::spawn_sand();
        cells[pixel_pos] = Cell::spawn_empty();

    // Down water
    } else if cells[down].state == CellState::Water {
        cells[down] = Cell::spawn_sand();
        cells[pixel_pos] = Cell::spawn_water();

    // Down left
    } else if downleft_is_empty {
        cells[down_left] = Cell::spawn_sand();
        cells[pixel_pos] = Cell::spawn_empty();

    // Down right
    } else if downright_is_empty {
        cells[down_right] = Cell::spawn_sand();
        cells[pixel_pos] = Cell::spawn_empty();
    }
}

pub async fn update_water(x: usize, y: usize, cells: &mut [Cell]) {
    let pixel_pos: usize = (y * GRID_X_SIZE) + x;

    let down: usize = if y < GRID_Y_SIZE - 1 {
        pixel_pos + GRID_X_SIZE
    } else {
        pixel_pos
    };

    let down_left: usize = if x > 0 { down - 1 } else { pixel_pos };

    let down_right: usize = if x < GRID_X_SIZE - 1 {
        down + 1
    } else {
        pixel_pos
    };

    let left: usize = if x > 0 { pixel_pos - 1 } else { pixel_pos };

    let right: usize = if x < GRID_X_SIZE - 1 {
        pixel_pos + 1
    } else {
        pixel_pos
    };

    //Down-Side checker
    let downleft_is_empty = cells[down_left] == Cell::spawn_empty();
    let downright_is_empty = cells[down_right] == Cell::spawn_empty();
    //Side checker
    let left_is_empty = cells[left] == Cell::spawn_empty();
    let right_is_empty = cells[right] == Cell::spawn_empty();

    //Down
    if cells[down] == Cell::spawn_empty() {
        cells[down] = Cell::spawn_water();
        cells[pixel_pos] = Cell::spawn_empty();

    //Down left
    } else if x != 0 && downleft_is_empty {
        cells[down_left] = Cell::spawn_water();
        cells[pixel_pos] = Cell::spawn_empty();

    //Down right
    } else if x != GRID_X_SIZE - 1 && downright_is_empty {
        cells[down_right] = Cell::spawn_water();
        cells[pixel_pos] = Cell::spawn_empty();
    //Left
    } else if x != 0 && left_is_empty && cells[pixel_pos].move_direction == Direction::Left {
        cells[left] = Cell::spawn_water();
        cells[left].move_direction = Direction::Left;
        cells[pixel_pos] = Cell::spawn_empty();
    //Right
    } else if x != GRID_X_SIZE - 1
        && right_is_empty
        && cells[pixel_pos].move_direction == Direction::Right
    {
        cells[right] = Cell::spawn_water();
        cells[right].move_direction = Direction::Right;
        cells[pixel_pos] = Cell::spawn_empty();
    } else {
        match cells[pixel_pos].move_direction {
            Direction::Left => cells[pixel_pos].move_direction = Direction::Right,
            Direction::Right => cells[pixel_pos].move_direction = Direction::Left,
            Direction::None => (),
        }
    }
}
