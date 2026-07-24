use crate::{
    client::client_message::ClientMessage, server::server_message::ServerMessage,
    states::app_state::AppState,
};

use macroquad::prelude::*;

use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

// ============================================================================
// CLIENT UTILITIES
// ============================================================================

/// Function to process incoming messages from the server and manage automated replies
pub fn process_server_messages(
    tcp_stream: &mut Option<TcpStream>,
    logs: &mut Vec<String>,
    current_player_id: &mut String,
    current_game_id: &mut String,
    buffer: &mut String,
    game_started: &mut bool,
) {
    if let Some(ref mut stream) = *tcp_stream {
        let mut temp_buf = [0; 1024];

        match stream.read(&mut temp_buf) {
            Ok(0) => {
                logs.push("[SUCCESS] Server closed the connection.".to_string());
            }
            Ok(n) => {
                let received_str = std::str::from_utf8(&temp_buf[..n]).unwrap();
                buffer.push_str(received_str); // Append received string to buffer

                // Loop through buffer looking for '\n' to parse complete messages
                while let Some(pos) = buffer.find('\n') {
                    let message_str = buffer[..pos].to_string();
                    *buffer = buffer[pos + 1..].to_string();

                    // Attempt to parse the incoming JSON as a ServerMessage
                    match serde_json::from_str::<ServerMessage>(&message_str) {
                        Ok(ServerMessage::JoinAccepted {
                            player_id, game_id, ..
                        }) => {
                            logs.push("[SUCCESS] Received JOIN_ACCEPTED message.".to_string());
                            *current_player_id = player_id;
                            *current_game_id = game_id;
                        }
                        Ok(ServerMessage::JoinRejected { reason, .. }) => {
                            logs.push(format!("[ERROR] JOIN_REJECTED by server: {}", reason));
                        }
                        Ok(ServerMessage::GameStarted {
                            game_id,
                            rows,
                            columns,
                            ..
                        }) => {
                            logs.push(format!(
                                "[SUCCESS] Received GAME_STARTED for game {} (Grid: {}x{}).",
                                game_id, rows, columns
                            ));
                            *game_started = true;
                        }
                        Ok(ServerMessage::GameState { tick, players, .. }) => {
                            logs.push(format!(
                                "[INFO] Received GAME_STATE tick #{} (Players: {}).",
                                tick,
                                players.len()
                            ));
                        }
                        Ok(ServerMessage::FlagPickedUp {
                            tick, player_id, ..
                        }) => {
                            logs.push(format!(
                                "[INFO] Flag picked up by {} at tick #{}.",
                                player_id, tick
                            ));
                        }
                        Ok(ServerMessage::FlagStolen {
                            tick,
                            previous_carrier_id,
                            new_carrier_id,
                            ..
                        }) => {
                            logs.push(format!(
                                "[INFO] Flag stolen from {} by {} at tick #{}.",
                                previous_carrier_id, new_carrier_id, tick
                            ));
                        }
                        Ok(ServerMessage::PlayerDisconnected { player_id, .. }) => {
                            logs.push(format!(
                                "[WARN] Player {} disconnected from game.",
                                player_id
                            ));
                        }
                        Ok(ServerMessage::GameOver {
                            winner_id,
                            winner_name,
                            reason,
                            ..
                        }) => {
                            logs.push(format!(
                                "[SUCCESS] GAME OVER! Winner: {} ({}) - Reason: {}",
                                winner_name, winner_id, reason
                            ));
                        }
                        Ok(ServerMessage::Error {
                            code, description, ..
                        }) => {
                            logs.push(format!("[ERROR] Server Error [{}]: {}", code, description));
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
    }
}

/// Function to capture user inputs and send corresponding actions to the server
pub fn handle_client_input(
    tcp_stream: &mut Option<TcpStream>,
    logs: &mut Vec<String>,
    app_state: &mut AppState,
    current_player_id: &str,
    current_game_id: &str,
    game_started: &bool,
) {
    // Handle ESCAPE to leave the game
    if is_key_pressed(KeyCode::Escape) {
        if let Some(mut stream) = tcp_stream.take() {
            if !current_player_id.is_empty() && !current_game_id.is_empty() {
                let leave_msg = ClientMessage::leave(current_game_id, current_player_id);
                let _ = stream.write_all(leave_msg.as_bytes());
            }
            logs.push("[SUCCESS] Correct closure of the connection.".to_string());
        }
        *app_state = AppState::MainMenu;
        return; // Exit early since connection is dropped
    }

    // Handle Directional Input (Only if the game has started and IDs are valid)
    if *game_started && !current_player_id.is_empty() && !current_game_id.is_empty() {
        let mut new_direction = None;

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            new_direction = Some("UP");
        } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            new_direction = Some("DOWN");
        } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            new_direction = Some("LEFT");
        } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            new_direction = Some("RIGHT");
        }

        // Send the movement payload if a direction key was pressed
        if let Some(dir) = new_direction {
            if let Some(ref mut stream) = *tcp_stream {
                let dir_msg =
                    ClientMessage::change_direction(current_game_id, current_player_id, dir);
                if stream.write_all(dir_msg.as_bytes()).is_ok() {
                    logs.push(format!("[INFO] Sent direction change: {}", dir));
                } else {
                    logs.push("[ERROR] Failed to send direction to server.".to_string());
                }
            }
        }
    }
}
