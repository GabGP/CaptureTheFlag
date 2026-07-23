use crate::{
    app_state::AppState,
    board::Board,
    config::*,
    game_state::GameState,
    server::server_message::{PlayerState, ServerMessage},
};

use macroquad::{
    prelude::*,
    rand::gen_range,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

// ============================================================================
// GAME LOBBY (SERVER)
// ============================================================================

/// Renders the server lobby UI and handles initialization actions.
pub fn draw_lobby(
    tcp_listener: &mut Option<TcpListener>,
    active_clients: &mut Vec<TcpStream>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
    game_state: &mut GameState,
    board: &mut Option<Board>,
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    root_ui().window(
        hash!(),
        vec2(center_x - 140., center_y - 120.),
        vec2(280., 240.),
        |ui| {
            ui.label(None, "--- SERVER LOBBY ---");
            ui.label(None, &format!("Status: {:?}", game_state));
            ui.label(
                None,
                &format!("Connected Clients: {}", active_clients.len()),
            );

            // Option 1: START GAME (Only enabled while in WAITING state)
            if *game_state == GameState::Waiting {
                if ui.button(None, "START GAME") {
                    logs.push("[INFO] Starting game sequence...".to_string());
                    *game_state = GameState::Starting;

                    // Generate Board with Flag and Obstacles
                    let mut new_board = Board::generate();

                    // Assign starting positions for connected players
                    let mut initial_players: Vec<PlayerState> = Vec::new();
                    for (index, _) in active_clients.iter().enumerate() {
                        let p_id = format!("P{:02}", index + 1);
                        let start_col = gen_range(0, new_board.columns);

                        initial_players.push(PlayerState {
                            player_id: p_id,
                            name: format!("Player {}", index + 1),
                            row: -1,
                            column: start_col,
                            direction: "DOWN".to_string(),
                            inside_board: false,
                            has_flag: false,
                            protected: false,
                        });
                    }

                    // Build GAME_STARTED payload
                    let game_started_msg = ServerMessage::game_started(
                        "GAME-001",
                        new_board.rows,
                        new_board.columns,
                        MOVEMENT_INTERVAL_MS,
                        PROTECTION_TIME_MS,
                        new_board.obstacles.clone(),
                        new_board.flag.clone(),
                        initial_players.clone(),
                    );

                    // Broadcast GAME_STARTED to all connected clients
                    for stream in active_clients.iter_mut() {
                        let _ = stream.write_all(game_started_msg.as_bytes());
                    }

                    // Save the generated players into the board
                    new_board.players = initial_players;

                    *board = Some(new_board);
                    *game_state = GameState::Running;
                    logs.push("[SUCCESS] GAME_STARTED broadcast sent!".to_string());
                }
            }

            // Option 2: CANCEL GAME & SHUT DOWN SERVER
            if ui.button(None, "CANCEL & SHUTDOWN") {
                *game_state = GameState::Cancelled;
                *tcp_listener = None;
                active_clients.clear();

                logs.push("[INFO] Server shutdown and returned to Main Menu.".to_string());
                *app_state = AppState::MainMenu;
            }
        },
    );
}
