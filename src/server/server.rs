use crate::{
    app_state::AppState,
    board::Board,
    game_state::GameState,
    server::{lobby::draw_lobby, network_helpers::*},
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

use std::net::{TcpListener, TcpStream};

// ============================================================================
// SERVER INITIALIZATION
// ============================================================================

/// Function to start the server
pub fn server_start(
    ip_input: &mut String,
    port_input: &mut String,
    tcp_listener: &mut Option<TcpListener>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    root_ui().window(
        hash!(),
        vec2(center_x - 125., center_y - 100.),
        vec2(250., 200.),
        |ui| {
            ui.label(None, "--- SERVER CONFIGURATION ---");
            ui.input_text(hash!(), "IP", ip_input);
            ui.input_text(hash!(), "PORT", port_input);

            if ui.button(None, "START SERVER") {
                let bind_addr = format!("{}:{}", ip_input, port_input);

                match TcpListener::bind(&bind_addr) {
                    Ok(listener) => {
                        // Must be non-blocking so Macroquad doesn't freeze
                        listener.set_nonblocking(true).unwrap();
                        *tcp_listener = Some(listener);
                        logs.push(format!("[SUCCESS] Server started on {}", bind_addr));
                        *app_state = AppState::ServerRunning;
                    }
                    Err(e) => logs.push(format!("[ERROR] {}", e)),
                }
            }
            if ui.button(None, "BACK") {
                *app_state = AppState::MainMenu;
            }
        },
    );
}

// ============================================================================
// SERVER MAIN RUNNING LOOP
// ============================================================================

/// Function to handle the running server loop
pub fn server_running(
    tcp_listener: &mut Option<TcpListener>,
    active_clients: &mut Vec<TcpStream>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
    game_state: &mut GameState,
    board: &mut Option<Board>,
) {
    draw_text(
        "Server running...",
        20.0,
        20.0,
        20.0,
        YELLOW,
    );

    draw_lobby(
        tcp_listener,
        active_clients,
        logs,
        app_state,
        game_state,
        board,
    );

    // Handle Network Operations
    accept_new_connections(tcp_listener, active_clients, logs, game_state);
    let disconnected_indices = process_client_messages(active_clients, logs, game_state);
    cleanup_disconnected_clients(active_clients, disconnected_indices);
}
