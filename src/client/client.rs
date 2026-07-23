use crate::{
    app_state::AppState,
    client::{client_message::ClientMessage, network_helpers::process_server_messages},
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

use std::{io::Write, net::TcpStream};

// ============================================================================
// CLIENT INITIALIZATION
// ============================================================================

/// Function to start the client
pub fn client_start(
    ip_input: &mut String,
    port_input: &mut String,
    name_input: &mut String,
    tcp_stream: &mut Option<TcpStream>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
) {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    root_ui().window(
        hash!(),
        vec2(center_x - 125., center_y - 120.),
        vec2(250., 240.),
        |ui| {
            ui.label(None, "--- CLIENT CONFIGURATION ---");
            ui.input_text(hash!(), "IP", ip_input);
            ui.input_text(hash!(), "PORT", port_input);
            ui.input_text(hash!(), "NAME", name_input);

            if ui.button(None, "CONNECT TO SERVER") {
                let server_addr = format!("{}:{}", ip_input, port_input);

                match TcpStream::connect(&server_addr) {
                    Ok(stream) => {
                        // Must be non-blocking so Macroquad doesn't freeze
                        stream.set_nonblocking(true).unwrap();
                        *tcp_stream = Some(stream);
                        logs.push("[SUCCESS] 1. Connected to the server.".to_string());

                        let join_msg = ClientMessage::join(&name_input);

                        if let Some(ref mut active_stream) = *tcp_stream {
                            active_stream.write_all(join_msg.as_bytes()).unwrap();
                            logs.push("[SUCCESS] 2. Sent JOIN message.".to_string());
                        }
                        *app_state = AppState::ClientRunning;
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
// CLIENT MAIN RUNNING LOOP
// ============================================================================

/// Function to handle the running client
pub fn client_running(
    tcp_stream: &mut Option<TcpStream>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
    current_player_id: &mut String,
    current_game_id: &mut String,
    buffer: &mut String,
    direction_sent: &mut bool,
) {
    draw_text(
        "Press ESCAPE to send LEAVE and close connection.",
        20.0,
        20.0,
        20.0,
        YELLOW,
    );

    // Handle Client Input
    if is_key_pressed(KeyCode::Escape) {
        if let Some(mut stream) = tcp_stream.take() {
            if !current_player_id.is_empty() && !current_game_id.is_empty() {
                let leave_msg = ClientMessage::leave(&current_game_id, &current_player_id);
                let _ = stream.write_all(leave_msg.as_bytes());
            }
            logs.push("[SUCCESS] 7. Correct closure of the connection.".to_string());
        }
        *app_state = AppState::MainMenu;
    }

    // Process Incoming TCP Data
    process_server_messages(
        tcp_stream,
        logs,
        current_player_id,
        current_game_id,
        buffer,
        direction_sent,
    );
}
