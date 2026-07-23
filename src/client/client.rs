use crate::{
    app_state::AppState, client::client_message::ClientMessage,
    server::server_message::ServerMessage,
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

// ============================================================================
// CLIENT
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
    if let Some(ref mut stream) = *tcp_stream {
        let mut temp_buf = [0; 1024];
        match stream.read(&mut temp_buf) {
            Ok(0) => {
                logs.push("[SUCCESS] Server closed the connection.".to_string());
            }
            Ok(n) => {
                let received_str = std::str::from_utf8(&temp_buf[..n]).unwrap();
                buffer.push_str(received_str);

                while let Some(pos) = buffer.find('\n') {
                    let message_str = buffer[..pos].to_string();
                    *buffer = buffer[pos + 1..].to_string();

                    // Attempt to parse the incoming JSON as a ServerMessage
                    match serde_json::from_str::<ServerMessage>(&message_str) {
                        Ok(ServerMessage::JoinAccepted {
                            player_id, game_id, ..
                        }) => {
                            logs.push("[SUCCESS] 3. Received JOIN_ACCEPTED message.".to_string());
                            // Directly assign the strings extracted from the enum variant
                            *current_player_id = player_id;
                            *current_game_id = game_id;
                        }
                        Ok(ServerMessage::GameState { .. }) => {
                            logs.push("[SUCCESS] 5. Received GAME_STATE message.".to_string());
                        }
                        Ok(_) => {
                            // Placeholder to handle other messages
                            logs.push("[SUCCESS] Received other message type.".to_string());
                        }
                        Err(e) => {
                            logs.push(format!("[WARN] Failed to parse message: {}", e));
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => {
                logs.push(format!("[ERROR] {}", e));
            }
        }

        if !current_player_id.is_empty() && !*direction_sent {
            let dir_msg =
                ClientMessage::change_direction(&current_game_id, &current_player_id, "RIGHT");
            stream.write_all(dir_msg.as_bytes()).unwrap();
            logs.push("[SUCCESS] 4. Sent CHANGE_DIRECTION message.".to_string());
            *direction_sent = true;
        }
    }
}
