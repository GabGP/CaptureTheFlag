use crate::{
    app_state::AppState,
    client::client_message::ClientMessage,
    server::server_message::{Flag, ServerMessage},
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
    window::{screen_height, screen_width},
};

use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

// ============================================================================
// SERVER
// ============================================================================

/// Function to start the server
pub fn server_start(
    ip_input: &mut String,
    port_input: &mut String,
    tcp_listener: &mut Option<TcpListener>,
    logs: &mut Vec<String>,
    state: &mut AppState,
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
                        *state = AppState::ServerRunning;
                    }
                    Err(e) => logs.push(format!("[ERROR] {}", e)),
                }
            }
            if ui.button(None, "BACK") {
                *state = AppState::MainMenu;
            }
        },
    );
}

/// Function to handle the running server
pub fn server_running(
    tcp_listener: &mut Option<TcpListener>,
    active_clients: &mut Vec<TcpStream>,
    logs: &mut Vec<String>,
    state: &mut AppState,
) {
    draw_text(
        "Server running... Press ESCAPE to stop.",
        20.0,
        20.0,
        20.0,
        YELLOW,
    );

    // Accept new incoming connections
    if let Some(ref listener) = *tcp_listener {
        match listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true).unwrap();
                logs.push(format!("[INFO] New connection from {}", addr.ip()));
                active_clients.push(stream);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => logs.push(format!("[ERROR] {}", e)),
        }
    }

    // Process messages from active clients
    let mut disconnected_indices = Vec::new();

    for (i, stream) in active_clients.iter_mut().enumerate() {
        let mut buf = [0; 1024];

        match stream.read(&mut buf) {
            Ok(0) => {
                // Reading 0 bytes means the client gracefully closed the connection
                logs.push(format!("[INFO] Client at index {} disconnected.", i));
                disconnected_indices.push(i);
            }
            Ok(n) => {
                let msg_str = String::from_utf8_lossy(&buf[..n]);

                // Clients may send multiple consecutive messages, split by newline
                for line in msg_str.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    // Attempt to parse the incoming JSON as a ClientMessage
                    match serde_json::from_str::<ClientMessage>(trimmed) {
                        Ok(ClientMessage::Join { name, .. }) => {
                            logs.push(format!("[INFO] JOIN received from: {}", name));

                            // Server sends JOIN_ACCEPTED
                            let response = ServerMessage::join_accepted("P01", "GAME-001");
                            let _ = stream.write_all(response.as_bytes());
                        }
                        Ok(ClientMessage::ChangeDirection { direction, .. }) => {
                            logs.push(format!("[INFO] CHANGE_DIRECTION received: {}", direction));

                            // Server sends GAME_STATE
                            let dummy_flag = Flag {
                                row: 10,
                                column: 11,
                                status: "AVAILABLE".to_string(),
                                carrier_id: None,
                            };

                            let response = ServerMessage::game_state(
                                "GAME-001",
                                1,      // dummy tick
                                vec![], // dummy empty players list
                                dummy_flag,
                            );
                            let _ = stream.write_all(response.as_bytes());
                        }
                        Ok(ClientMessage::Leave { .. }) => {
                            logs.push("[INFO] LEAVE received.".to_string());
                            disconnected_indices.push(i);
                        }
                        Err(e) => {
                            logs.push(format!("[WARN] Failed to parse message: {}", e));
                        }
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                // No incoming data at this exact frame.
            }
            Err(e) => {
                logs.push(format!("[ERROR] Client connection lost: {}", e));
                disconnected_indices.push(i);
            }
        }
    }

    // Clean up disconnected clients
    // Iterate in reverse so removing an index doesn't shift the remaining target indices
    for i in disconnected_indices.into_iter().rev() {
        if i < active_clients.len() {
            active_clients.remove(i);
        }
    }

    // Handle manual server shutdown
    if is_key_pressed(KeyCode::Escape) {
        *tcp_listener = None;
        active_clients.clear(); // Drop all TcpStreams, closing the sockets
        logs.push("[SUCCESS] Server stopped.".to_string());
        *state = AppState::MainMenu;
    }
}
