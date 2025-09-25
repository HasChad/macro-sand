use macroquad::prelude::*;

use crate::{AppState, DOT_SIZE_IN_PXS, GRID_X_SIZE};

pub async fn draw_world(state: &mut AppState) {
    // Per-pixel coloring
    for (i, cell) in state.buffer.iter_mut().enumerate() {
        state.image.set_pixel(
            (i % GRID_X_SIZE) as u32,
            (i / GRID_X_SIZE) as u32,
            cell.color,
        );

        state.cells[i] = *cell;
    }

    // Rendering
    state.texture.update(&state.image);
    state.texture.set_filter(FilterMode::Nearest);

    draw_texture_ex(
        &state.texture,
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

    // Cursor
    let (mouse_xpos, mouse_ypos) = mouse_position();
    let m_posx = mouse_xpos as u32;
    let m_posy = mouse_ypos as u32;

    draw_rectangle_lines(
        m_posx as f32,
        m_posy as f32,
        (state.brush.size * DOT_SIZE_IN_PXS) as f32,
        (state.brush.size * DOT_SIZE_IN_PXS) as f32,
        2.0,
        WHITE,
    );
}
