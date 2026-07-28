use crate::{
    config::*,
    gui::{
        camera::Camera2DWorld,
        render::render_game_world,
        render_logs::render_event_logs,
        render_overlays::{render_countdown_overlay, render_game_over_overlay, render_go_burst},
        ui::{gui_button, gui_panel},
    },
    protocol::types::*,
    server::server::GameServer,
};
use macroquad::prelude::*;

use super::runner::AppMode;

// ============================================================================
// MODE SERVER
// ============================================================================

/// Function to monitor server state, manage match lifecycle, and render host world view
pub fn update(server: &mut GameServer, camera: &mut Camera2DWorld, time: f32) -> Option<AppMode> {
    let snap = server.update_snapshot();
    let config = GameConfig::default();

    camera.target_x = 0.0;
    camera.target_y = 0.0;
    camera.zoom = (screen_height() / (config.map_size * 1.15)).clamp(0.1, 0.8);

    // 1. Render World
    render_game_world(
        camera,
        &config,
        &snap.players,
        snap.flag_status,
        snap.flag_carrier_id,
        snap.flag_x,
        snap.flag_y,
        None,
        time,
    );

    let sw = screen_width();
    let sh = screen_height();

    // 2. Top-Left HUD Information
    gui_panel(20.0, 20.0, sw - 40.0, 60.0, "");

    let carrier_info = if snap.flag_status == FlagStatus::Carried {
        // Find the carrier's name from the players vector
        if let Some(player) = snap
            .players
            .iter()
            .find(|p| p.player_id == snap.flag_carrier_id)
        {
            format!("Carried by {}", player.name)
        } else {
            format!("Carried by ID {}", snap.flag_carrier_id)
        }
    } else {
        format!("{:?}", snap.flag_status)
    };

    // Server Info
    draw_text(
        &format!(
            "SERVER: {} ({}) | STATE: {:?} | GAME ID: {} | SERVER MODE",
            snap.server_name, snap.server_ip, snap.state, snap.game_id,
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

    // 3. Connected Players HUD
    let players_count = snap.players.len();
    let hud_w = 240.0;
    let hud_h = 135.0 + (players_count as f32 * 22.0);

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
    for p in &snap.players {
        draw_text(
            &format!("- [ID: {}] {}", p.player_id, p.name),
            35.0,
            py,
            FONT_SIZE_SMALL,
            WHITE,
        );
        py += 22.0;
    }

    // 4. Start Match / Leave Buttons
    if snap.state == GameState::Waiting {
        if gui_button(
            sw - 260.0,
            30.0,
            220.0,
            40.0,
            "[>] START MATCH",
            Color::from_rgba(0, 200, 100, 255),
        ) {
            server.start_countdown();
        }

        if gui_button(
            20.0,
            screen_height() - 50.0,
            150.0,
            36.0,
            "[<] GO BACK",
            Color::from_rgba(200, 60, 60, 255),
        ) {
            return Some(AppMode::Launcher);
        }
    } else {
        if gui_button(
            sw - 260.0,
            30.0,
            220.0,
            40.0,
            "[<] LEAVE SERVER",
            Color::from_rgba(200, 60, 60, 255),
        ) {
            return Some(AppMode::Launcher);
        }
    }

    render_event_logs(sw - 340.0, 90.0, 320.0, 300.0, &snap.logs);

    // Countdown Overlay & "GO!" Burst
    if snap.state == GameState::Starting {
        render_countdown_overlay(snap.countdown_seconds, time, sw, sh);
    } else if snap.state == GameState::Running && snap.tick <= 15 {
        render_go_burst(snap.tick, sw, sh);
    }

    // Game Over Overlay
    if snap.state == GameState::Finished {
        if render_game_over_overlay(&snap.winner_name, sw, sh) {
            return Some(AppMode::Launcher);
        }
    }

    None
}
