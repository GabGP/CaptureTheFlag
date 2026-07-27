use crate::{
    protocol::{
        network::tcp_utils::read_frame,
        protocol::{Message, PROTOCOL_VERSION},
        types::*,
    },
    server::{
        server_events::handle_network_event,
        server_game::{update_countdown_state, update_running_game_state},
        server_net::{ClientNetEvent, broadcast_msg},
        server_utils::{ServerCommand, ServerStateSnapshot},
    },
};
use std::collections::HashMap;
use std::io;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================================
// SERVER LOOP
// ============================================================================

/// Function to run the main background server loop managing network, events, and game state[cite: 24]
pub fn run_server_loop(
    config: GameConfig,
    server_name: String,
    tcp_listener: TcpListener,
    udp_socket: Option<UdpSocket>,
    cmd_rx: Receiver<ServerCommand>,
    snap_tx: Sender<ServerStateSnapshot>,
    latest_snapshot_arc: Arc<Mutex<ServerStateSnapshot>>,
) -> io::Result<()> {
    let game_id: u16 = 1001;
    let (net_tx, net_rx) = channel::<ClientNetEvent>();

    let mut state = GameState::Waiting;
    let mut next_player_id: u16 = 1;
    let mut players: HashMap<u16, PlayerState> = HashMap::new();
    let mut player_streams: HashMap<u16, TcpStream> = HashMap::new();

    let mut flag_status = FlagStatus::Available;
    let mut flag_carrier_id: u16 = 0;
    let mut flag_x: f32 = 0.0;
    let mut flag_y: f32 = 0.0;
    let mut tick: u32 = 0;
    let mut countdown_sec = config.countdown_seconds;
    let mut last_countdown_instant = Instant::now();
    let mut logs = vec![format!(
        "Server running on port TCP {} / UDP {}",
        config.server_port, config.discovery_port
    )];

    let mut pending_inputs: HashMap<u16, Direction> = HashMap::new();
    let mut pending_interacts: HashMap<u16, Instant> = HashMap::new();

    // Winner
    let mut winner_id: Option<u16> = None;
    let mut winner_name: String = String::new();

    // Tick
    let tick_dur = Duration::from_millis(config.tick_interval_ms as u64);
    let mut last_tick_time = Instant::now();

    loop {
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                ServerCommand::StartCountdown => {
                    if state == GameState::Waiting {
                        state = GameState::Starting;
                        countdown_sec = config.countdown_seconds;
                        last_countdown_instant = Instant::now();
                        logs.push(format!("Countdown started ({} seconds)...", countdown_sec));
                        let msg = Message::GameCountdown {
                            seconds_remaining: countdown_sec,
                        };
                        broadcast_msg(&mut player_streams, &msg);
                    }
                }
                ServerCommand::StopServer => return Ok(()),
            }
        }

        if let Some(ref sock) = udp_socket {
            let mut buf = [0u8; 512];
            while let Ok((len, src)) = sock.recv_from(&mut buf) {
                if len >= 2 && buf[0] == 0x01 && buf[1] == PROTOCOL_VERSION {
                    if state == GameState::Waiting {
                        let resp = Message::DiscoverResponse {
                            game_id,
                            server_name: server_name.clone(),
                            tcp_port: config.server_port,
                            state: GameState::Waiting,
                            player_count: players.len() as u16,
                            maximum_players: config.maximum_players,
                        };
                        let bytes = resp.serialize();
                        let _ = sock.send_to(&bytes, src);
                    }
                }
            }
        }

        loop {
            match tcp_listener.accept() {
                Ok((stream, _addr)) => {
                    let net_tx_clone = net_tx.clone();
                    thread::spawn(move || {
                        let mut stream = stream;
                        stream.set_nonblocking(false).ok();
                        stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
                        if let Ok(Message::Join { name }) = read_frame(&mut stream) {
                            let _ = net_tx_clone.send(ClientNetEvent::Join { name, stream });
                        }
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        while let Ok(event) = net_rx.try_recv() {
            handle_network_event(
                event,
                &mut state,
                &mut next_player_id,
                &mut players,
                &mut player_streams,
                &mut logs,
                &net_tx,
                game_id,
                &config,
                &mut flag_status,
                &mut flag_carrier_id,
                &mut flag_x,
                &mut flag_y,
                &mut pending_inputs,
                &mut pending_interacts,
            );
        }

        update_countdown_state(
            &mut state,
            &mut countdown_sec,
            &mut last_countdown_instant,
            &mut tick,
            &mut players,
            &mut player_streams,
            &mut flag_status,
            &mut flag_carrier_id,
            &mut flag_x,
            &mut flag_y,
            &mut logs,
            &config,
            &mut winner_id,
            &mut winner_name,
        );

        update_running_game_state(
            &mut state,
            &mut tick,
            &mut players,
            &mut player_streams,
            &mut flag_status,
            &mut flag_carrier_id,
            &mut flag_x,
            &mut flag_y,
            &mut logs,
            &config,
            &mut pending_inputs,
            &mut pending_interacts,
            &mut last_tick_time,
            tick_dur,
            &mut winner_id,
            &mut winner_name,
        );

        let current_snap = ServerStateSnapshot {
            state,
            game_id,
            players: players.values().cloned().collect(),
            flag_status,
            flag_carrier_id,
            flag_x,
            flag_y,
            tick,
            countdown_seconds: countdown_sec,
            logs: logs.clone(),
            server_name: server_name.clone(),
            server_ip: format!("0.0.0.0:{}", config.server_port),
            winner_id,
            winner_name: winner_name.clone(),
        };
        *latest_snapshot_arc.lock().unwrap() = current_snap.clone();
        let _ = snap_tx.send(current_snap);

        thread::sleep(Duration::from_millis(5));
    }
}
