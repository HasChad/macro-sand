use macroquad::prelude::*;

use crate::{cells::Cell, AppState, DOT_SIZE_IN_PXS, GRID_X_SIZE};

pub async fn update_brush(state: &mut AppState) {
    let (m_xpos, m_ypos) = mouse_position();

    if is_mouse_button_down(MouseButton::Left) {
        if m_xpos >= 0. && m_xpos < screen_width() && m_ypos >= 0. && m_ypos < screen_height() {
            let brush_size = 3;

            let posx = m_xpos as usize / DOT_SIZE_IN_PXS - (brush_size - 1) / 2;
            let posy = m_ypos as usize / DOT_SIZE_IN_PXS - (brush_size - 1) / 2;

            for y in 0..brush_size {
                for x in 0..brush_size {
                    let pixel_pos = posx + x + ((posy + y) * GRID_X_SIZE);

                    if state.cells[pixel_pos] == Cell::empty() {
                        state.cells[pixel_pos] = state.brush;
                    }
                }
            }
        }
    }

    if is_mouse_button_down(MouseButton::Right) {
        if m_xpos >= 0. && m_xpos < screen_width() && m_ypos >= 0. && m_ypos < screen_height() {
            let brush_size = 3;

            let posx = m_xpos as usize / DOT_SIZE_IN_PXS - (brush_size - 1) / 2;
            let posy = m_ypos as usize / DOT_SIZE_IN_PXS - (brush_size - 1) / 2;

            for y in 0..brush_size {
                for x in 0..brush_size {
                    let pixel_pos = posx + x + ((posy + y) * GRID_X_SIZE);

                    state.buffer[pixel_pos] = Cell::empty();
                }
            }
        }
    }
}
