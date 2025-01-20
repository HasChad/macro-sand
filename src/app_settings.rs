use macroquad::prelude::*;

use crate::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Macro-Sand".into(),
        window_width: (GRID_X_SIZE * DOT_SIZE_IN_PXS) as i32,
        window_height: (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
