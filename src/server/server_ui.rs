use crate::{server::server::GameServer, states::app_state::AppState, types::*};
use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

// ============================================================================
// SERVER UI
// ============================================================================

/// Function to start the server configuration via Macroquad UI
pub fn server_start(
    port_input: &mut String,
    discovery_port_input: &mut String,
    server_name_input: &mut String,
    game_server: &mut Option<GameServer>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    root_ui().window(
        hash!(),
        vec2(center_x - 140., center_y - 140.),
        vec2(280., 280.),
        |ui| {
            ui.label(None, "--- SERVER CONFIGURATION ---");
            ui.input_text(hash!(), "PORT", port_input);
            ui.input_text(hash!(), "DISCOVERY PORT", discovery_port_input);
            ui.input_text(hash!(), "SERVER NAME", server_name_input);

            if ui.button(None, "START SERVER") {
                let port = port_input.trim().parse::<u16>().unwrap_or(5000);
                let disc_port = discovery_port_input.trim().parse::<u16>().unwrap_or(5001);
                let s_name = if server_name_input.is_empty() {
                    "Game Server".to_string()
                } else {
                    server_name_input.clone()
                };

                let mut config = GameConfig::default();
                config.server_port = port;
                config.discovery_port = disc_port;

                match GameServer::start(config, s_name) {
                    Ok(server) => {
                        *game_server = Some(server);
                        logs.push(format!("[SUCCESS] Server started on port {}", port));
                        *app_state = AppState::ServerRunning;
                    }
                    Err(e) => {
                        logs.push(format!("[ERROR] Failed to start server: {}", e));
                    }
                }
            }
            if ui.button(None, "BACK") {
                *app_state = AppState::MainMenu;
            }
        },
    );
}

/// Function to handle the running server loop and lobby UI
pub fn server_running(
    game_server: &mut Option<GameServer>,
    app_state: &mut AppState,
    logs: &mut Vec<String>,
) {
    draw_text("Server running...", 20.0, 20.0, 20.0, YELLOW);

    let snapshot = if let Some(server) = game_server {
        server.update_snapshot()
    } else {
        return;
    };

    for l in &snapshot.logs {
        if !logs.contains(l) {
            logs.push(l.clone());
        }
    }

    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    root_ui().window(
        hash!(),
        vec2(center_x - 150., center_y - 160.),
        vec2(300., 320.),
        |ui| {
            ui.label(None, "--- SERVER LOBBY ---");
            ui.label(None, &format!("Status: {:?}", snapshot.state));
            ui.label(
                None,
                &format!("Connected Players: {}", snapshot.players.len()),
            );
            ui.label(None, &format!("Tick: {}", snapshot.tick));

            if snapshot.state == GameState::Waiting {
                if ui.button(None, "START COUNTDOWN") {
                    if let Some(server) = game_server {
                        server.start_countdown();
                        logs.push("[INFO] Starting countdown sequence...".to_string());
                    }
                }
            } else if snapshot.state == GameState::Starting {
                ui.label(
                    None,
                    &format!("Starting in: {}s", snapshot.countdown_seconds),
                );
            }

            if ui.button(None, "CANCEL & SHUTDOWN") {
                *game_server = None;
                logs.push("[INFO] Server shutdown and returned to Main Menu.".to_string());
                *app_state = AppState::MainMenu;
            }
        },
    );
}
