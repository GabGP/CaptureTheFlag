use crate::{
    protocol::{network::tcp_utils::send_frame, protocol::Message},
    server::server_net::{ClientNetEvent, broadcast_msg, spawn_client_reader},
    types::*,
};
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::time::Instant;

// ============================================================================
// SERVER EVENTS
// ============================================================================

/// Function to process incoming client network events like joins, inputs, and disconnections
pub fn handle_network_event(
    event: ClientNetEvent,
    state: &mut GameState,
    next_player_id: &mut u16,
    players: &mut HashMap<u16, PlayerState>,
    player_streams: &mut HashMap<u16, TcpStream>,
    logs: &mut Vec<String>,
    net_tx: &Sender<ClientNetEvent>,
    game_id: u16,
    config: &GameConfig,
    flag_status: &mut FlagStatus,
    flag_carrier_id: &mut u16,
    flag_x: &mut f32,
    flag_y: &mut f32,
    pending_inputs: &mut HashMap<u16, Direction>,
    pending_interacts: &mut HashMap<u16, Instant>,
) {
    match event {
        ClientNetEvent::Join { name, mut stream } => {
            let trimmed_name = name.trim().to_string();

            if trimmed_name.is_empty() || trimmed_name.len() > 20 {
                let _ = send_frame(
                    &mut stream,
                    &Message::JoinRejected {
                        reason: JoinRejectReason::InvalidName,
                    },
                );
                return;
            }

            if *state != GameState::Waiting {
                let _ = send_frame(
                    &mut stream,
                    &Message::JoinRejected {
                        reason: JoinRejectReason::GameAlreadyStarted,
                    },
                );
                return;
            }

            if players.len() >= config.maximum_players as usize {
                let _ = send_frame(
                    &mut stream,
                    &Message::JoinRejected {
                        reason: JoinRejectReason::GameFull,
                    },
                );
                return;
            }

            let pid = *next_player_id;
            *next_player_id += 1;

            if send_frame(
                &mut stream,
                &Message::JoinAccepted {
                    player_id: pid,
                    game_id,
                },
            )
            .is_ok()
            {
                let player_obj = PlayerState {
                    player_id: pid,
                    name: trimmed_name.clone(),
                    x: 0.0,
                    y: 0.0,
                    direction: Direction::None,
                    has_flag: false,
                };
                players.insert(pid, player_obj);
                logs.push(format!("Player '{}' (ID {}) joined.", trimmed_name, pid));

                spawn_client_reader(stream.try_clone().unwrap(), net_tx.clone(), pid);
                player_streams.insert(pid, stream);

                let lobby_players: Vec<LobbyPlayer> = players
                    .values()
                    .map(|p| LobbyPlayer {
                        player_id: p.player_id,
                        name: p.name.clone(),
                    })
                    .collect();
                let lobby_msg = Message::LobbyState {
                    state: *state,
                    players: lobby_players,
                };
                broadcast_msg(player_streams, &lobby_msg);
            }
        }
        ClientNetEvent::Input {
            player_id,
            direction,
        } => {
            pending_inputs.insert(player_id, direction);
        }
        ClientNetEvent::Interact { player_id } => {
            pending_interacts
                .entry(player_id)
                .or_insert_with(Instant::now);
        }
        ClientNetEvent::Leave { player_id } | ClientNetEvent::Disconnected { player_id } => {
            if let Some(removed) = players.remove(&player_id) {
                player_streams.remove(&player_id);
                logs.push(format!(
                    "Player '{}' (ID {}) disconnected.",
                    removed.name, player_id
                ));

                if *flag_carrier_id == player_id {
                    *flag_status = FlagStatus::Dropped;
                    *flag_carrier_id = 0;
                    *flag_x = removed.x;
                    *flag_y = removed.y;
                    logs.push(format!("Flag dropped at ({:.1}, {:.1})", *flag_x, *flag_y));
                }

                broadcast_msg(player_streams, &Message::PlayerDisconnected { player_id });

                if *state == GameState::Waiting {
                    let lobby_players: Vec<LobbyPlayer> = players
                        .values()
                        .map(|p| LobbyPlayer {
                            player_id: p.player_id,
                            name: p.name.clone(),
                        })
                        .collect();
                    broadcast_msg(
                        player_streams,
                        &Message::LobbyState {
                            state: *state,
                            players: lobby_players,
                        },
                    );
                }
            }
        }
    }
}
