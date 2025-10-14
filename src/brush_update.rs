use macroquad::prelude::*;

use crate::{cells::Cell, AppState, DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};

pub async fn update_brush(state: &mut AppState) {
    if state.clear {
        state.buffer.fill(Cell::empty());
        state.clear = false;
    }

    let (m_xpos, m_ypos) = mouse_position();

    if is_mouse_button_down(MouseButton::Left) {
        if m_xpos >= 0. && m_xpos < screen_width() && m_ypos >= 0. && m_ypos < screen_height() {
            let posx = m_xpos as usize / DOT_SIZE_IN_PXS;
            let posy = m_ypos as usize / DOT_SIZE_IN_PXS;

            for pos in state.brush.pos.iter() {
                let pixel_pos = posx + pos.x as usize + ((posy + pos.y as usize) * GRID_X_SIZE);

                if pixel_pos < GRID_X_SIZE * GRID_Y_SIZE {
                    if state.buffer[pixel_pos] == Cell::empty() {
                        state.buffer[pixel_pos] = state.brush.brush_type;
                    }
                }
            }
        }
    }

    if is_mouse_button_down(MouseButton::Right) {
        if m_xpos >= 0. && m_xpos < screen_width() && m_ypos >= 0. && m_ypos < screen_height() {
            let posx = m_xpos as usize / DOT_SIZE_IN_PXS;
            let posy = m_ypos as usize / DOT_SIZE_IN_PXS;

            for pos in state.brush.pos.iter() {
                let pixel_pos = posx + pos.x as usize + ((posy + pos.y as usize) * GRID_X_SIZE);

                if pixel_pos < GRID_X_SIZE * GRID_Y_SIZE {
                    state.buffer[pixel_pos] = Cell::empty();
                }
            }
        }
    }
}
