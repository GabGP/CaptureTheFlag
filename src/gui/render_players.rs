use crate::{gui::camera::Camera2DWorld, protocol::types::*};
use macroquad::prelude::*;

// ============================================================================
// RENDER PLAYERS
// ============================================================================

/// Function to get the player color based on their ID
pub fn get_player_color(id: u16) -> Color {
    let colors = [
        Color::from_rgba(239, 83, 80, 255),
        Color::from_rgba(66, 165, 245, 255),
        Color::from_rgba(102, 187, 106, 255),
        Color::from_rgba(171, 71, 188, 255),
        Color::from_rgba(255, 167, 38, 255),
        Color::from_rgba(38, 198, 218, 255),
        Color::from_rgba(236, 64, 122, 255),
        Color::from_rgba(212, 225, 87, 255),
    ];
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
            draw_circle(px, py, p_radius + 6.0, Color::from_rgba(255, 255, 255, 120));
        }

        draw_circle(px, py, p_radius, p_color);
        draw_circle_lines(px, py, p_radius, 2.0, WHITE);

        let dir_len = p_radius * 1.6;
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
            draw_line(px, py, px + dx, py + dy, 3.0, WHITE);
            draw_circle(px + dx, py + dy, 3.0, WHITE);
        }

        let name_str = if p.name.is_empty() {
            format!("P{:02}", p.player_id)
        } else {
            format!("{} (P{:02})", p.name, p.player_id)
        };
        let dims = measure_text(&name_str, None, 14, 1.0);
        let tag_x = px - dims.width / 2.0;
        let tag_y = py - p_radius - 8.0;

        draw_rectangle(
            tag_x - 4.0,
            tag_y - 12.0,
            dims.width + 8.0,
            16.0,
            Color::from_rgba(0, 0, 0, 180),
        );
        let text_color = if p.has_flag {
            GOLD
        } else if is_local {
            GREEN
        } else {
            WHITE
        };
        draw_text(&name_str, tag_x, tag_y, 14.0, text_color);
    }
}
