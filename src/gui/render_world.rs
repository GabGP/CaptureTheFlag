use crate::{gui::camera::Camera2DWorld, protocol::types::*};
use macroquad::prelude::*;

// ============================================================================
// RENDER WORLD
// ============================================================================

/// Function to render the map background and grid
pub fn render_background_and_grid(cam: &Camera2DWorld, config: &GameConfig) {
    let half_map = config.map_size / 2.0;
    clear_background(Color::from_rgba(15, 20, 32, 255));

    let (min_x, min_y) = cam.world_to_screen(-half_map, -half_map);
    let (max_x, max_y) = cam.world_to_screen(half_map, half_map);
    let map_w = max_x - min_x;
    let map_h = max_y - min_y;

    draw_rectangle(
        min_x,
        min_y,
        map_w,
        map_h,
        Color::from_rgba(22, 30, 48, 255),
    );
    draw_rectangle_lines(
        min_x,
        min_y,
        map_w,
        map_h,
        3.0,
        Color::from_rgba(64, 120, 200, 180),
    );

    let step = 200.0;
    let mut gx = -half_map + step;
    while gx < half_map {
        let (sx1, sy1) = cam.world_to_screen(gx, -half_map);
        let (sx2, sy2) = cam.world_to_screen(gx, half_map);
        draw_line(sx1, sy1, sx2, sy2, 1.0, Color::from_rgba(255, 255, 255, 12));
        gx += step;
    }
    let mut gy = -half_map + step;
    while gy < half_map {
        let (sx1, sy1) = cam.world_to_screen(-half_map, gy);
        let (sx2, sy2) = cam.world_to_screen(half_map, gy);
        draw_line(sx1, sy1, sx2, sy2, 1.0, Color::from_rgba(255, 255, 255, 12));
        gy += step;
    }
}

/// Function to render the central capture circle
pub fn render_central_circle(cam: &Camera2DWorld, config: &GameConfig, time: f32) {
    let (cx, cy) = cam.world_to_screen(0.0, 0.0);
    let circle_r_screen = cam.world_to_screen_dist(config.circle_radius);

    draw_circle(cx, cy, circle_r_screen, Color::from_rgba(0, 229, 255, 20));
    let pulse = (time * 3.0).sin() * 0.2 + 0.8;
    let cyan_glow = Color::from_rgba(0, 229, 255, (180.0 * pulse) as u8);
    draw_circle_lines(cx, cy, circle_r_screen, 4.0, cyan_glow);

    let label = "CENTRAL CIRCLE (CAPTURE ZONE)";
    let font_size = (18.0 * cam.zoom).clamp(12.0, 22.0);
    let dims = measure_text(label, None, font_size as u16, 1.0);
    draw_text(
        label,
        cx - dims.width / 2.0,
        cy - circle_r_screen - 10.0,
        font_size,
        Color::from_rgba(0, 229, 255, 220),
    );
}

/// Function to render the flag and its status
pub fn render_flag(
    cam: &Camera2DWorld,
    config: &GameConfig,
    flag_status: FlagStatus,
    flag_x: f32,
    flag_y: f32,
    time: f32,
) {
    if flag_status == FlagStatus::Outside {
        return;
    }
    let (fx, fy) = cam.world_to_screen(flag_x, flag_y);
    let gold_pulse = (time * 4.0).sin() * 0.3 + 0.7;

    if flag_status == FlagStatus::Available || flag_status == FlagStatus::Dropped {
        let glow_r = cam.world_to_screen_dist(config.interaction_radius * 0.5) * gold_pulse;
        draw_circle(fx, fy, glow_r, Color::from_rgba(255, 215, 0, 40));
        draw_circle(
            fx,
            fy,
            cam.world_to_screen_dist(12.0),
            Color::from_rgba(255, 215, 0, 255),
        );
        draw_circle_lines(
            fx,
            fy,
            cam.world_to_screen_dist(config.interaction_radius),
            1.5,
            Color::from_rgba(255, 215, 0, 100),
        );

        let pole_h = cam.world_to_screen_dist(35.0);
        draw_line(fx, fy, fx, fy - pole_h, 3.0, WHITE);
        draw_triangle(
            Vec2::new(fx, fy - pole_h),
            Vec2::new(
                fx + cam.world_to_screen_dist(25.0),
                fy - pole_h + cam.world_to_screen_dist(10.0),
            ),
            Vec2::new(fx, fy - pole_h + cam.world_to_screen_dist(20.0)),
            GOLD,
        );

        let txt = if flag_status == FlagStatus::Available {
            "FLAG (AVAILABLE)"
        } else {
            "FLAG (DROPPED)"
        };
        draw_text(txt, fx - 40.0, fy + 25.0, 14.0, GOLD);
    } else if flag_status == FlagStatus::Carried {
        let glow_r = cam.world_to_screen_dist(config.player_radius * 2.2) * gold_pulse;
        draw_circle(fx, fy, glow_r, Color::from_rgba(255, 215, 0, 80));
        draw_circle_lines(fx, fy, glow_r, 2.5, GOLD);

        let pole_h = cam.world_to_screen_dist(40.0);
        draw_line(fx, fy, fx, fy - pole_h, 3.0, GOLD);
        draw_triangle(
            Vec2::new(fx, fy - pole_h),
            Vec2::new(
                fx + cam.world_to_screen_dist(22.0),
                fy - pole_h + cam.world_to_screen_dist(8.0),
            ),
            Vec2::new(fx, fy - pole_h + cam.world_to_screen_dist(16.0)),
            YELLOW,
        );
    }
}
