use crate::{config::*, gui::camera::Camera2DWorld, protocol::types::*};
use macroquad::prelude::*;

// ============================================================================
// RENDER PLAYERS
// ============================================================================

/// Function to get the player color based on their ID
pub fn get_player_color(id: u16) -> Color {
    let colors = PLAYER_COLORS;
    colors[(id as usize) % colors.len()]
}

/// Function to render all players on the screen
pub fn render_players(
    cam: &Camera2DWorld,
    config: &GameConfig,
    players: &[PlayerState],
    local_player_id: Option<u16>,
) {
    for p in players {
        let (px, py) = cam.world_to_screen(p.x, p.y);
        let p_radius = cam.world_to_screen_dist(config.player_radius);
        let is_local = local_player_id == Some(p.player_id);
        let p_color = get_player_color(p.player_id);

        if is_local {
            draw_circle(
                px,
                py,
                p_radius + RENDER_PLAYER_AURA_OFFSET,
                COLOR_PLAYER_LOCAL_AURA,
            );
        }

        draw_circle(px, py, p_radius, p_color);
        draw_circle_lines(px, py, p_radius, RENDER_PLAYER_OUTLINE_THICKNESS, WHITE);

        let dir_len = p_radius * RENDER_PLAYER_DIR_MULTIPLIER;
        let mut dx = 0.0;
        let mut dy = 0.0;
        match p.direction {
            Direction::Up => dy = -dir_len,
            Direction::Down => dy = dir_len,
            Direction::Left => dx = -dir_len,
            Direction::Right => dx = dir_len,
            Direction::None => {}
        }
        if p.direction != Direction::None {
            draw_line(
                px,
                py,
                px + dx,
                py + dy,
                RENDER_PLAYER_DIR_LINE_THICKNESS,
                WHITE,
            );
            draw_circle(px + dx, py + dy, RENDER_PLAYER_DIR_DOT_RADIUS, WHITE);
        }

        // NAME TAG
        let name_str = if p.name.is_empty() {
            format!("P{:02}", p.player_id)
        } else {
            format!("{} (P{:02})", p.name, p.player_id)
        };
        let dims = measure_text(&name_str, None, FONT_SIZE_MEDIUM as u16, 1.0);
        let tag_x = px - dims.width / 2.0;
        let tag_y = py - p_radius - UI_NAME_TAG_OFFSET_Y;

        draw_rectangle(
            tag_x - UI_NAME_TAG_PADDING_X,
            tag_y - UI_NAME_TAG_PADDING_Y,
            dims.width + UI_NAME_TAG_OFFSET_Y,
            UI_NAME_TAG_HEIGHT,
            COLOR_NAME_TAG_BG,
        );
        let text_color = if p.has_flag {
            GOLD
        } else if is_local {
            GREEN
        } else {
            WHITE
        };
        draw_text(&name_str, tag_x, tag_y, FONT_SIZE_MEDIUM, text_color);
    }
}
