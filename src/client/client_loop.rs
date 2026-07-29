use crate::{
    client::client_utils::{ClientCommand, ClientStateSnapshot},
    protocol::network::tcp_utils::{read_frame, send_frame},
    protocol::{protocol::Message, types::*},
};
use std::io;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============================================================================
// CLIENT LOOP
// ============================================================================

/// Function to run the background network event and message handling loop
pub fn run_client_loop(
    target_ip: String,
    target_port: u16,
    player_name: String,
    cmd_rx: Receiver<ClientCommand>,
    state_writer: Arc<Mutex<ClientStateSnapshot>>,
) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:{}", target_ip, target_port))?;
    stream.set_nonblocking(false)?;

    let remote_addr = format!("{}:{}", target_ip, target_port);
    let mut writer = stream.try_clone()?;

    send_frame(
        &mut writer,
        &Message::Join { name: player_name },
        "client",
        &remote_addr,
    )?;

    let first_resp = read_frame(&mut stream, "client", &remote_addr)?;
    let mut my_player_id = 0;
    let mut _my_game_id = 0;

    match first_resp {
        Message::JoinAccepted { player_id, game_id } => {
            my_player_id = player_id;
            _my_game_id = game_id;
            let mut snap = state_writer.lock().unwrap();
            snap.connected = true;
            snap.player_id = player_id;
            snap.game_id = game_id;
            snap.logs
                .push(format!("Joined server successfully! ID: {}", player_id));
        }
        Message::JoinRejected { reason } => {
            let mut snap = state_writer.lock().unwrap();
            snap.error_msg = Some(format!("Join rejected by server: {:?}", reason));
            return Ok(());
        }
        _ => {
            let mut snap = state_writer.lock().unwrap();
            snap.error_msg = Some("Unexpected initial server response".to_string());
            return Ok(());
        }
    }

    stream.set_nonblocking(true)?;

    loop {
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                ClientCommand::SendInput(dir) => {
                    let msg = Message::Input {
                        player_id: my_player_id,
                        direction: dir,
                    };
                    let _ = send_frame(&mut writer, &msg, "client", &remote_addr);
                }
                ClientCommand::SendInteract => {
                    let msg = Message::Interact {
                        player_id: my_player_id,
                    };
                    let _ = send_frame(&mut writer, &msg, "client", &remote_addr);
                }
                ClientCommand::Leave => {
                    let msg = Message::Leave {
                        player_id: my_player_id,
                    };
                    let _ = send_frame(&mut writer, &msg, "client", &remote_addr);

                    // Shut down the TCP connection after sending LEAVE
                    let _ = stream.shutdown(std::net::Shutdown::Both);

                    let mut snap = state_writer.lock().unwrap();
                    snap.connected = false;
                    snap.logs.push("Left game.".to_string());
                    return Ok(());
                }
            }
        }

        match read_frame(&mut stream, "client", &remote_addr) {
            Ok(msg) => match msg {
                Message::LobbyState { state, players } => {
                    let mut snap = state_writer.lock().unwrap();
                    snap.game_state = state;
                    snap.lobby_players = players.clone();

                    // Rebuild player_names from the active lobby list
                    snap.player_names =
                        players.into_iter().map(|p| (p.player_id, p.name)).collect();
                }
                Message::GameCountdown { seconds_remaining } => {
                    let mut snap = state_writer.lock().unwrap();
                    snap.game_state = GameState::Starting;
                    snap.countdown_seconds = seconds_remaining;
                    snap.logs.push(format!("Countdown: {}", seconds_remaining));
                }
                Message::GameStarted {
                    config,
                    flag_status,
                    flag_carrier_id,
                    flag_x,
                    flag_y,
                    players,
                } => {
                    let mut snap = state_writer.lock().unwrap();
                    snap.game_state = GameState::Running;
                    snap.config = config;
                    snap.flag_status = flag_status;
                    snap.flag_carrier_id = flag_carrier_id;
                    snap.flag_x = flag_x;
                    snap.flag_y = flag_y;
                    snap.players = players.clone();
                    for p in players {
                        snap.player_names.insert(p.player_id, p.name);
                    }
                    snap.logs.push("Match started!".to_string());
                }
                Message::GameStateMsg {
                    tick,
                    flag_status,
                    flag_carrier_id,
                    flag_x,
                    flag_y,
                    players,
                } => {
                    let mut snap = state_writer.lock().unwrap();
                    if tick >= snap.tick {
                        snap.tick = tick;
                        snap.flag_status = flag_status;
                        snap.flag_carrier_id = flag_carrier_id;
                        snap.flag_x = flag_x;
                        snap.flag_y = flag_y;

                        let updated_players: Vec<PlayerState> = players
                            .into_iter()
                            .map(|mut p| {
                                if let Some(n) = snap.player_names.get(&p.player_id) {
                                    p.name = n.clone();
                                }
                                p
                            })
                            .collect();

                        snap.players = updated_players;
                    }
                }
                Message::FlagPickedUp { tick: _, player_id } => {
                    let mut snap = state_writer.lock().unwrap();
                    let pname = snap
                        .player_names
                        .get(&player_id)
                        .cloned()
                        .unwrap_or_else(|| format!("ID {}", player_id));
                    snap.logs.push(format!("Flag picked up by {}!", pname));
                }
                Message::FlagStolen {
                    tick: _,
                    previous_carrier_id,
                    new_carrier_id,
                } => {
                    let mut snap = state_writer.lock().unwrap();
                    let old_p = snap
                        .player_names
                        .get(&previous_carrier_id)
                        .cloned()
                        .unwrap_or_else(|| format!("ID {}", previous_carrier_id));
                    let new_p = snap
                        .player_names
                        .get(&new_carrier_id)
                        .cloned()
                        .unwrap_or_else(|| format!("ID {}", new_carrier_id));
                    snap.logs
                        .push(format!("Flag stolen by {} from {}!", new_p, old_p));
                }
                Message::PlayerDisconnected { player_id } => {
                    let mut snap = state_writer.lock().unwrap();

                    // Remove from player_names HashMap
                    let pname = snap
                        .player_names
                        .remove(&player_id)
                        .unwrap_or_else(|| format!("ID {}", player_id));

                    // Clean up from both player lists
                    snap.players.retain(|p| p.player_id != player_id);
                    snap.lobby_players.retain(|p| p.player_id != player_id);

                    snap.logs.push(format!("Player {} disconnected.", pname));
                }
                Message::GameOver {
                    winner_id,
                    winner_name,
                    reason: _,
                } => {
                    let mut snap = state_writer.lock().unwrap();
                    snap.game_state = GameState::Finished;
                    snap.winner_id = Some(winner_id);
                    snap.winner_name = winner_name.clone();
                    snap.logs.push(format!("WINNER: {}!", winner_name));
                }
                Message::ErrorMsg { code, description } => {
                    let mut snap = state_writer.lock().unwrap();
                    snap.logs
                        .push(format!("Server Error ({:?}): {}", code, description));
                }
                _ => {}
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(_) => {
                let mut snap = state_writer.lock().unwrap();
                snap.connected = false;
                snap.error_msg = Some("Connection lost to server".to_string());
                break;
            }
        }
    }

    Ok(())
}
