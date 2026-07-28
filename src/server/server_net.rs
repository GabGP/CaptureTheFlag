use crate::protocol::{
    network::tcp_utils::{read_frame, send_frame},
    protocol::Message,
    types::*,
};
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

// ============================================================================
// SERVER NETWORK
// ============================================================================

pub enum ClientNetEvent {
    Join {
        name: String,
        stream: TcpStream,
    },
    Input {
        player_id: u16,
        direction: Direction,
    },
    Interact {
        player_id: u16,
    },
    Leave {
        player_id: u16,
    },
    Disconnected {
        player_id: u16,
    },
}

/// Function to broadcast a message to all connected client streams and prune disconnected ones
pub fn broadcast_msg(streams: &mut HashMap<u16, TcpStream>, msg: &Message) {
    let mut to_remove = Vec::new();
    for (pid, stream) in streams.iter_mut() {
        let address = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        if send_frame(stream, msg, "server", &address).is_err() {
            to_remove.push(*pid);
        }
    }
    for pid in to_remove {
        streams.remove(&pid);
    }
}

/// Function to spawn a reader thread for an accepted client stream
pub fn spawn_client_reader(stream: TcpStream, net_tx: Sender<ClientNetEvent>, pid: u16) {
    if let Ok(stream_clone) = stream.try_clone() {
        let net_tx_clone = net_tx.clone();
        std::thread::spawn(move || {
            let mut stream = stream_clone;
            stream.set_nonblocking(false).ok();
            stream.set_read_timeout(None).ok();
            let peer_address = stream
                .peer_addr()
                .map(|a| a.to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            loop {
                match read_frame(&mut stream, "server", &peer_address) {
                    Ok(Message::Input {
                        player_id,
                        direction,
                    }) => {
                        let _ = net_tx_clone.send(ClientNetEvent::Input {
                            player_id,
                            direction,
                        });
                    }
                    Ok(Message::Interact { player_id }) => {
                        let _ = net_tx_clone.send(ClientNetEvent::Interact { player_id });
                    }
                    Ok(Message::Leave { player_id }) => {
                        let _ = net_tx_clone.send(ClientNetEvent::Leave { player_id });
                        break;
                    }
                    Ok(_) => {}
                    Err(_) => {
                        let _ = net_tx_clone.send(ClientNetEvent::Disconnected { player_id: pid });
                        break;
                    }
                }
            }
        });
    }
}
