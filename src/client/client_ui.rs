use crate::{client::client::GameClient, states::app_state::AppState, types::*};
use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

// ============================================================================
// CLIENT UI
// ============================================================================

/// Function to start the client configuration via Macroquad UI
pub fn client_start(
    ip_input: &mut String,
    port_input: &mut String,
    name_input: &mut String,
    game_client: &mut Option<GameClient>,
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
            ui.label(None, "--- CLIENT CONFIGURATION ---");
            ui.input_text(hash!(), "IP", ip_input);
            ui.input_text(hash!(), "PORT", port_input);
            ui.input_text(hash!(), "NAME", name_input);

            if ui.button(None, "CONNECT TO SERVER") {
                let ip = if ip_input.is_empty() {
                    "127.0.0.1".to_string()
                } else {
                    ip_input.clone()
                };
                let port = port_input.trim().parse::<u16>().unwrap_or(5000);
                let name = if name_input.is_empty() {
                    "Player".to_string()
                } else {
                    name_input.clone()
                };

                match GameClient::connect(ip, port, name) {
                    Ok(client) => {
                        *game_client = Some(client);
                        logs.push("[SUCCESS] Connected to server handler.".to_string());
                        *app_state = AppState::ClientRunning;
                    }
                    Err(e) => {
                        logs.push(format!("[ERROR] Connection failed: {}", e));
                    }
                }
            }
            if ui.button(None, "BACK") {
                *app_state = AppState::MainMenu;
            }
        },
    );
}

/// Function to handle the running client loop and UI
pub fn client_running(
    game_client: &mut Option<GameClient>,
    app_state: &mut AppState,
    logs: &mut Vec<String>,
) {
    draw_text(
        "Press ESCAPE to leave and return to Main Menu.",
        20.0,
        20.0,
        20.0,
        YELLOW,
    );

    let client = if let Some(c) = game_client {
        c
    } else {
        *app_state = AppState::MainMenu;
        return;
    };

    let snapshot = client.get_snapshot();

    for l in &snapshot.logs {
        if !logs.contains(l) {
            logs.push(l.clone());
        }
    }

    if let Some(err) = &snapshot.error_msg {
        logs.push(format!("[CLIENT ERROR] {}", err));
        *game_client = None;
        *app_state = AppState::MainMenu;
        return;
    }

    if is_key_pressed(KeyCode::Escape) {
        client.leave();
        *game_client = None;
        *app_state = AppState::MainMenu;
        return;
    }

    let mut new_dir = Direction::None;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        new_dir = Direction::Up;
    } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        new_dir = Direction::Down;
    } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        new_dir = Direction::Left;
    } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        new_dir = Direction::Right;
    }
    client.set_direction(new_dir);

    if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Space) {
        client.send_interact();
    }

    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    root_ui().window(
        hash!(),
        vec2(center_x - 160., center_y - 180.),
        vec2(320., 360.),
        |ui| {
            ui.label(None, "--- GAME CLIENT ---");
            ui.label(None, &format!("Game State: {:?}", snapshot.game_state));
            ui.label(None, &format!("Player ID: {}", snapshot.player_id));
            ui.label(
                None,
                &format!("Connected Players: {}", snapshot.players.len()),
            );
            ui.label(None, &format!("Tick: {}", snapshot.tick));

            if snapshot.game_state == GameState::Waiting {
                ui.label(None, "Waiting for host to start...");
                ui.separator();
                ui.label(None, "Lobby Players:");
                for p in &snapshot.lobby_players {
                    ui.label(None, &format!("- {} (ID: {})", p.name, p.player_id));
                }
            } else if snapshot.game_state == GameState::Starting {
                ui.label(
                    None,
                    &format!("Starting in {}s...", snapshot.countdown_seconds),
                );
            } else if snapshot.game_state == GameState::Running {
                ui.label(None, "Match in progress! Use WASD/Arrows & Space.");
                ui.label(None, &format!("Flag Status: {:?}", snapshot.flag_status));
            } else if snapshot.game_state == GameState::Finished {
                ui.label(
                    None,
                    &format!("GAME OVER! Winner: {}", snapshot.winner_name),
                );
            }
        },
    );
}
