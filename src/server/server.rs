use crate::{server::server_utils::*, types::*};
use std::io;
use std::net::{TcpListener, UdpSocket};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// SERVER
// ============================================================================

pub struct GameServer {
    pub config: GameConfig,
    cmd_tx: Sender<ServerCommand>,
    snapshot_rx: Receiver<ServerStateSnapshot>,
    pub latest_snapshot: Arc<Mutex<ServerStateSnapshot>>,
}

impl GameServer {
    /// Function to initialize and start the game server background thread
    pub fn start(config: GameConfig, server_name: String) -> io::Result<Self> {
        let (cmd_tx, cmd_rx) = channel::<ServerCommand>();
        let (snap_tx, snap_rx) = channel::<ServerStateSnapshot>();

        let mut logs = vec!["Server initialized.".to_string()];

        let tcp_listener =
            TcpListener::bind(format!("0.0.0.0:{}", config.server_port)).map_err(|e| {
                io::Error::new(e.kind(), format!("TCP port {}: {}", config.server_port, e))
            })?;
        tcp_listener.set_nonblocking(true)?;

        let udp_socket = match UdpSocket::bind(format!("0.0.0.0:{}", config.discovery_port)) {
            Ok(sock) => {
                sock.set_nonblocking(true)?;
                Some(sock)
            }
            Err(e) => {
                eprintln!(
                    "[SERVER WARN] UDP {} unavailable: {}",
                    config.discovery_port, e
                );
                logs.push(format!(
                    "WARNING: UDP {} in use ({}). Server browser disabled -",
                    config.discovery_port, e
                ));
                logs.push("clients must use DIRECT JOIN BY IP & PORT.".to_string());
                None
            }
        };

        let initial_snapshot = ServerStateSnapshot {
            state: GameState::Waiting,
            game_id: 1001,
            players: Vec::new(),
            flag_status: FlagStatus::Available,
            flag_carrier_id: 0,
            flag_x: 0.0,
            flag_y: 0.0,
            tick: 0,
            countdown_seconds: config.countdown_seconds,
            logs,
        };

        let latest_snapshot = Arc::new(Mutex::new(initial_snapshot.clone()));
        let snapshot_writer = latest_snapshot.clone();

        let cfg = config.clone();
        thread::spawn(move || {
            if let Err(e) = super::server_loop::run_server_loop(
                cfg,
                server_name,
                tcp_listener,
                udp_socket,
                cmd_rx,
                snap_tx,
                snapshot_writer.clone(),
            ) {
                eprintln!("[SERVER ERROR] {}", e);
                snapshot_writer
                    .lock()
                    .unwrap()
                    .logs
                    .push(format!("SERVER ERROR: {}", e));
            }
        });

        Ok(Self {
            config,
            cmd_tx,
            snapshot_rx: snap_rx,
            latest_snapshot,
        })
    }

    /// Function to trigger the game start countdown sequence
    pub fn start_countdown(&self) {
        let _ = self.cmd_tx.send(ServerCommand::StartCountdown);
    }

    /// Function to update and retrieve the latest server state snapshot
    pub fn update_snapshot(&mut self) -> ServerStateSnapshot {
        while let Ok(snap) = self.snapshot_rx.try_recv() {
            *self.latest_snapshot.lock().unwrap() = snap;
        }
        self.latest_snapshot.lock().unwrap().clone()
    }
}

impl Drop for GameServer {
    /// Function to automatically signal server shutdown on drop
    fn drop(&mut self) {
        let _ = self.cmd_tx.send(ServerCommand::StopServer);
    }
}
