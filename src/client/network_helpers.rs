use crate::{
    client::client_message::ClientMessage,
    server::server_message::ServerMessage,
};

use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

// ============================================================================
// CLIENT NETWORK HELPERS
// ============================================================================

/// Function to process incoming messages from the server and manage automated replies
pub fn process_server_messages(
    tcp_stream: &mut Option<TcpStream>,
    logs: &mut Vec<String>,
    current_player_id: &mut String,
    current_game_id: &mut String,
    buffer: &mut String,
    direction_sent: &mut bool,
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

                    // Attempt to parse the incoming JSON as a ServerMessages
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

        // Send initial direction once player is registered[cite: 30]
        if !current_player_id.is_empty() && !*direction_sent {
            let dir_msg =
                ClientMessage::change_direction(&current_game_id, &current_player_id, "RIGHT");
            stream.write_all(dir_msg.as_bytes()).unwrap();
            logs.push("[SUCCESS] 4. Sent CHANGE_DIRECTION message.".to_string());
            *direction_sent = true;
        }
    }
}