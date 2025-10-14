use ::rand::random_bool;
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellType {
    Dead,
    Sand,
    Water,
    Stone,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Right,
    Left,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellMatter {
    Solid,
    Liquid,
    //Gas,
    None,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cell {
    pub cell_type: CellType,
    pub matter: CellMatter,
    pub move_direction: Direction,
    pub is_moved: bool,
    pub color: Color,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            cell_type: CellType::Dead,
            matter: CellMatter::None,
            move_direction: Direction::None,
            is_moved: false,
            color: Color::from_rgba(10, 10, 10, 255),
        }
    }

    pub fn sand() -> Self {
        Self {
            cell_type: CellType::Sand,
            matter: CellMatter::Solid,
            move_direction: if random_bool(0.5) {
                Direction::Left
            } else {
                Direction::Right
            },
            is_moved: true,
            color: Color::from_rgba(255, 204, 92, 255),
        }
    }

    pub fn water() -> Self {
        Self {
            cell_type: CellType::Water,
            matter: CellMatter::Liquid,
            move_direction: if random_bool(0.5) {
                Direction::Left
            } else {
                Direction::Right
            },
            is_moved: true,
            color: Color::from_rgba(71, 140, 207, 255),
        }
    }

    pub fn stone() -> Self {
        Self {
            cell_type: CellType::Stone,
            matter: CellMatter::Solid,
            move_direction: Direction::None,
            is_moved: true,
            color: Color::from_rgba(151, 125, 139, 255),
        }
    }
}
