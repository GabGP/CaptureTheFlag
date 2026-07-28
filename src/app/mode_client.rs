use crate::{
    client::client::GameClient,
    config::*,
    gui::{
        camera::Camera2DWorld,
        render::render_game_world,
        render_logs::render_event_logs,
        render_overlays::{render_countdown_overlay, render_game_over_overlay, render_go_burst},
        ui::{gui_button, gui_panel},
    },
    protocol::types::*,
};
use macroquad::prelude::*;

use super::runner::AppMode;

// ============================================================================
// MODE CLIENT
// ============================================================================

/// Function to update client game state, render the HUD, and handle player inputs
pub fn update(client: &mut GameClient, camera: &mut Camera2DWorld, time: f32) -> Option<AppMode> {
    let snap = client.get_snapshot();

    // Word-wrapping implementation for long connection errors
    if let Some(ref err) = snap.error_msg {
        clear_background(Color::from_rgba(20, 15, 25, 255));
        gui_panel(
            screen_width() / 2.0 - 250.0,
            screen_height() / 2.0 - 150.0, // Expanded height to fit wrapped text
            500.0,
            300.0,
            "CONNECTION ERROR",
        );

        let mut err_y = screen_height() / 2.0 - 80.0;
        let mut current_line = String::new();

        for word in err.split_whitespace() {
            if current_line.len() + word.len() > 42 {
                draw_text(
                    &current_line,
                    screen_width() / 2.0 - 220.0,
                    err_y,
                    FONT_SIZE_MEDIUM,
                    COLOR_ERROR,
                );
                err_y += 25.0;
                current_line = String::new();
            }
            current_line.push_str(word);
            current_line.push(' ');
        }
        if !current_line.is_empty() {
            draw_text(
                &current_line,
                screen_width() / 2.0 - 220.0,
                err_y,
                FONT_SIZE_MEDIUM,
                COLOR_ERROR,
            );
        }

        if gui_button(
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + 80.0,
            200.0,
            40.0,
            "RETURN TO MENU",
            Color::from_rgba(0, 180, 255, 255),
        ) {
            return Some(AppMode::Launcher);
        }
        return None;
    }

    if let Some(me) = snap.players.iter().find(|p| p.player_id == snap.player_id) {
        camera.target_x = me.x;
        camera.target_y = me.y;
    }
    camera.zoom = 0.55;

    // 1. Render World & Logs
    render_game_world(
        camera,
        &snap.config,
        &snap.players,
        snap.flag_status,
        snap.flag_carrier_id,
        snap.flag_x,
        snap.flag_y,
        Some(snap.player_id),
        time,
    );
    render_event_logs(screen_width() - 340.0, 90.0, 320.0, 300.0, &snap.logs);

    // 2. Top-Left HUD Information
    let sw = screen_width();
    let sh = screen_height();

    gui_panel(20.0, 20.0, sw - 40.0, 60.0, "");
    let carrier_info = if snap.flag_status == FlagStatus::Carried {
        if let Some(name) = snap.player_names.get(&snap.flag_carrier_id) {
            format!("Carried by {}", name)
        } else {
            format!("Carried by ID {}", snap.flag_carrier_id)
        }
    } else {
        format!("{:?}", snap.flag_status)
    };

    // Server Info
    draw_text(
        &format!(
            "SERVER: {} ({}) | STATE: {:?} | GAME ID: {} | PLAYER ID: {}",
            snap.server_name, snap.server_ip, snap.game_state, snap.game_id, snap.player_id,
        ),
        35.0,
        45.0,
        FONT_SIZE_MEDIUM,
        WHITE,
    );
    draw_text(
        &format!("FLAG STATUS: {} | TICK: {}", carrier_info, snap.tick),
        35.0,
        65.0,
        FONT_SIZE_SMALL,
        COLOR_UI_ACCENT_CYAN,
    );

    // Calculate dynamic HUD size based on the number of connected players
    let players_count = snap.player_names.len();
    let hud_w = 240.0;
    let hud_h = 135.0 + (players_count as f32 * 22.0);

    // Connected Players HUD
    draw_rectangle(20.0, 90.0, hud_w, hud_h, Color::from_rgba(15, 20, 30, 200));
    draw_rectangle_lines(
        20.0,
        90.0,
        hud_w,
        hud_h,
        1.0,
        Color::from_rgba(60, 100, 150, 255),
    );

    draw_text(
        &format!("CONNECTED PLAYERS: {}", players_count),
        35.0,
        115.0,
        FONT_SIZE_REGULAR,
        Color::from_rgba(150, 180, 220, 255),
    );
    let mut py = 135.0;

    // Collect and sort by ID for a stable HUD layout
    let mut sorted_players: Vec<(&u16, &String)> = snap.player_names.iter().collect();
    sorted_players.sort_by_key(|&(id, _)| id);

    for (id, name) in sorted_players {
        let me_indicator = if *id == snap.player_id { " (You)" } else { "" };
        draw_text(
            &format!("- [ID: {}] {}{}", id, name, me_indicator),
            35.0,
            py,
            FONT_SIZE_SMALL,
            WHITE,
        );
        py += 22.0;
    }

    // 3. Bottom Controls Text
    let controls_txt =
        "CONTROLS:  [WASD / Arrow Keys] Move   |   [Space / E] Take or Steal Flag   |   [Esc] Exit";
    draw_text(
        controls_txt,
        sw / 2.0 - 360.0,
        sh - 20.0,
        FONT_SIZE_REGULAR,
        Color::from_rgba(220, 180, 0, 255), // Yellow
    );

    if snap.game_state == GameState::Running {
        let mut new_dir = Direction::None;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            new_dir = Direction::Up;
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            new_dir = Direction::Down;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            new_dir = Direction::Left;
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            new_dir = Direction::Right;
        }
        client.set_direction(new_dir);

        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::E) {
            client.send_interact();
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        client.leave();
        return Some(AppMode::Launcher);
    }

    // Countdown Overlay & "GO!" Burst
    if snap.game_state == GameState::Starting {
        render_countdown_overlay(snap.countdown_seconds, time, sw, sh);
    } else if snap.game_state == GameState::Running && snap.tick <= 15 {
        render_go_burst(snap.tick, sw, sh);
    }

    // Game Over Overlay
    if snap.game_state == GameState::Finished {
        if render_game_over_overlay(&snap.winner_name, sw, sh) {
            client.leave();
            return Some(AppMode::Launcher);
        }
    }

    None
}
