use crate::{
    config::*,
    gui::{
        camera::Camera2DWorld,
        render::render_game_world,
        render_logs::render_event_logs,
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
    gui_panel(20.0, 20.0, sw - 40.0, 70.0, "");
    draw_text(
        "SERVER MONITOR (HOST MODE)",
        35.0,
        45.0,
        FONT_SIZE_MEDIUM,
        COLOR_UI_ACCENT_CYAN,
    );

    if snap.state == GameState::Waiting {
        if gui_button(
            sw - 260.0,
            30.0,
            220.0,
            50.0,
            "[>] START MATCH",
            Color::from_rgba(0, 200, 100, 255),
        ) {
            server.start_countdown();
        }
    }

    render_event_logs(sw - 340.0, 100.0, 320.0, 350.0, &snap.logs);

    if gui_button(
        20.0,
        screen_height() - 50.0,
        150.0,
        36.0,
        "[<] LEAVE SERVER",
        Color::from_rgba(200, 60, 60, 255),
    ) {
        return Some(AppMode::Launcher);
    }

    None
}
