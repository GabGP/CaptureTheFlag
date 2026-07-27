use crate::config::*;
use macroquad::prelude::*;

// ============================================================================
// CAMERA
// ============================================================================

pub struct Camera2DWorld {
    pub target_x: f32,
    pub target_y: f32,
    pub zoom: f32,
}

impl Camera2DWorld {
    pub fn new() -> Self {
        Self {
            target_x: 0.0,
            target_y: 0.0,
            zoom: CAMERA_DEFAULT_ZOOM,
        }
    }

    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (f32, f32) {
        let sw = screen_width();
        let sh = screen_height();
        let sx = sw / 2.0 + (wx - self.target_x) * self.zoom;
        let sy = sh / 2.0 + (wy - self.target_y) * self.zoom;
        (sx, sy)
    }

    pub fn world_to_screen_dist(&self, dist: f32) -> f32 {
        dist * self.zoom
    }
}
