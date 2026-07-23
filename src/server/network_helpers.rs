use crate::{
    client::client_message::ClientMessage, game_state::GameState,
    server::server_message::ServerMessage,
};

use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

// ============================================================================
// NETWORK HELPERS
// ============================================================================

/// Function to accept new incoming connections from clients
pub fn accept_new_connections(
    tcp_listener: &mut Option<TcpListener>,
    active_clients: &mut Vec<TcpStream>,
    logs: &mut Vec<String>,
    game_state: &GameState,
) {
    // Accept new incoming connections ONLY if in WAITING game_state
    if *game_state == GameState::Waiting {
        if let Some(ref listener) = *tcp_listener {
            match listener.accept() {
                Ok((stream, addr)) => {
                    stream.set_nonblocking(true).unwrap();
                    logs.push(format!("[INFO] Connection accepted from {}", addr.ip()));
                    active_clients.push(stream);
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => logs.push(format!("[ERROR] {}", e)),
            }
        }
    }
}

/// Function to process incoming messages from connected clients
pub fn process_client_messages(
    active_clients: &mut [TcpStream],
    logs: &mut Vec<String>,
    game_state: &GameState,
) -> Vec<usize> {
    let mut disconnected_indices = Vec::new();

    for (i, stream) in active_clients.iter_mut().enumerate() {
        let mut buf = [0; 1024];

        match stream.read(&mut buf) {
            Ok(0) => {
                logs.push(format!("[INFO] Client {} disconnected.", i + 1));
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
                            if *game_state == GameState::Waiting {
                                // Server sends JOIN_ACCEPTED
                                logs.push(format!("[INFO] JOIN from: {}", name));
                                let player_id = format!("P{:02}", i + 1);
                                let response = ServerMessage::join_accepted(&player_id, "GAME-001");
                                let _ = stream.write_all(response.as_bytes());
                            } else {
                                // Server sends JOIN_REJECTED
                                let rejection =
                                    ServerMessage::join_rejected("GAME_ALREADY_STARTED");
                                let _ = stream.write_all(rejection.as_bytes());
                            }
                        }
                        Ok(ClientMessage::ChangeDirection { direction, .. }) => {
                            logs.push(format!("[INFO] Direction change: {}", direction));
                        }
                        Ok(ClientMessage::Leave { .. }) => {
                            logs.push(format!("[INFO] Client {} sent LEAVE.", i + 1));
                            disconnected_indices.push(i);
                        }
                        Err(e) => {
                            logs.push(format!("[WARN] Unrecognized message: {}", e));
                        }
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(e) => {
                logs.push(format!("[ERROR] Lost client connection: {}", e));
                disconnected_indices.push(i);
            }
        }
    }

    return disconnected_indices;
}

/// Function to clean up disconnected clients from the active_clients vector
pub fn cleanup_disconnected_clients(
    active_clients: &mut Vec<TcpStream>,
    disconnected_indices: Vec<usize>,
) {
    // Clean up disconnected clients in reverse
    for i in disconnected_indices.into_iter().rev() {
        if i < active_clients.len() {
            active_clients.remove(i);
        }
    }
}
