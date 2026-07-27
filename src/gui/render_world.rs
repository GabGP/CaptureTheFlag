use crate::{config::*, gui::camera::Camera2DWorld, protocol::types::*};
use macroquad::prelude::*;

// ============================================================================
// RENDER WORLD
// ============================================================================

/// Function to render the map background and grid
pub fn render_background_and_grid(cam: &Camera2DWorld, config: &GameConfig) {
    let half_map = config.map_size / 2.0;
    clear_background(COLOR_WORLD_BG_CLEAR);

    let (min_x, min_y) = cam.world_to_screen(-half_map, -half_map);
    let (max_x, max_y) = cam.world_to_screen(half_map, half_map);
    let map_w = max_x - min_x;
    let map_h = max_y - min_y;

    draw_rectangle(min_x, min_y, map_w, map_h, COLOR_WORLD_MAP_BG);
    draw_rectangle_lines(
        min_x,
        min_y,
        map_w,
        map_h,
        RENDER_MAP_BORDER_THICKNESS,
        COLOR_WORLD_MAP_BORDER,
    );

    // GRID
    let step = RENDER_GRID_STEP;
    let mut gx = -half_map + step;
    while gx < half_map {
        let (sx1, sy1) = cam.world_to_screen(gx, -half_map);
        let (sx2, sy2) = cam.world_to_screen(gx, half_map);
        draw_line(
            sx1,
            sy1,
            sx2,
            sy2,
            RENDER_GRID_LINE_THICKNESS,
            COLOR_WORLD_GRID_LINE,
        );
        gx += step;
    }
    let mut gy = -half_map + step;
    while gy < half_map {
        let (sx1, sy1) = cam.world_to_screen(-half_map, gy);
        let (sx2, sy2) = cam.world_to_screen(half_map, gy);
        draw_line(
            sx1,
            sy1,
            sx2,
            sy2,
            RENDER_GRID_LINE_THICKNESS,
            COLOR_WORLD_GRID_LINE,
        );
        gy += step;
    }
}

/// Function to render the central capture circle
pub fn render_central_circle(cam: &Camera2DWorld, config: &GameConfig, time: f32) {
    let (cx, cy) = cam.world_to_screen(0.0, 0.0);
    let circle_r_screen = cam.world_to_screen_dist(config.circle_radius);

    draw_circle(cx, cy, circle_r_screen, COLOR_CIRCLE_BG);
    let pulse = (time * 3.0).sin() * 0.2 + 0.8;
    let cyan_glow = Color::from_rgba(0, 229, 255, (180.0 * pulse) as u8);
    draw_circle_lines(
        cx,
        cy,
        circle_r_screen,
        RENDER_CIRCLE_BORDER_THICKNESS,
        cyan_glow,
    );

    let label = "CENTRAL CIRCLE (CAPTURE ZONE)";
    let font_size = (18.0 * cam.zoom).clamp(12.0, 22.0);
    let dims = measure_text(label, None, font_size as u16, 1.0);
    draw_text(
        label,
        cx - dims.width / 2.0,
        cy - circle_r_screen - RENDER_CIRCLE_TEXT_OFFSET_Y,
        font_size,
        COLOR_CIRCLE_TEXT,
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
        draw_circle(fx, fy, glow_r, COLOR_FLAG_GLOW_AVAILABLE);
        draw_circle(
            fx,
            fy,
            cam.world_to_screen_dist(12.0),
            COLOR_FLAG_CENTER_AVAILABLE,
        );
        draw_circle_lines(
            fx,
            fy,
            cam.world_to_screen_dist(config.interaction_radius),
            RENDER_FLAG_INTERACT_BORDER_THICKNESS,
            COLOR_FLAG_INTERACT_RADIUS,
        );

        // FLAG POLE
        let pole_h = cam.world_to_screen_dist(RENDER_FLAG_POLE_AVAILABLE_HEIGHT);
        draw_line(
            fx,
            fy,
            fx,
            fy - pole_h,
            RENDER_FLAG_POLE_LINE_THICKNESS,
            WHITE,
        );
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
        draw_text(
            txt,
            fx - RENDER_FLAG_POLE_CARRIED_HEIGHT,
            fy + 25.0,
            FONT_SIZE_MEDIUM,
            GOLD,
        );
    } else if flag_status == FlagStatus::Carried {
        let glow_r = cam.world_to_screen_dist(config.player_radius * 2.2) * gold_pulse;
        draw_circle(fx, fy, glow_r, COLOR_FLAG_GLOW_CARRIED);
        draw_circle_lines(fx, fy, glow_r, RENDER_FLAG_CARRIED_BORDER_THICKNESS, GOLD);

        // FLAG POLE
        let pole_h = cam.world_to_screen_dist(RENDER_FLAG_POLE_CARRIED_HEIGHT);
        draw_line(
            fx,
            fy,
            fx,
            fy - pole_h,
            RENDER_FLAG_POLE_LINE_THICKNESS,
            GOLD,
        );
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
