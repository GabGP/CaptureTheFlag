use crate::{client::client_utils::*, protocol::types::*};

use std::collections::HashMap;
use std::io;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// CLIENT
// ============================================================================

pub struct GameClient {
    cmd_tx: Sender<ClientCommand>,
    pub state_arc: Arc<Mutex<ClientStateSnapshot>>,
    pub active_direction: Direction,
}

impl GameClient {
    /// Function to establish connection with the server and initialize client state
    pub fn connect(target_ip: String, target_port: u16, player_name: String, server_name: String) -> io::Result<Self> {
        let (cmd_tx, cmd_rx) = channel::<ClientCommand>();

        let initial_snap = ClientStateSnapshot {
            connected: false,
            player_id: 0,
            game_id: 0,
            game_state: GameState::Waiting,
            lobby_players: Vec::new(),
            countdown_seconds: 0,
            config: GameConfig::default(),
            flag_status: FlagStatus::Available,
            flag_carrier_id: 0,
            flag_x: 0.0,
            flag_y: 0.0,
            tick: 0,
            players: Vec::new(),
            player_names: HashMap::new(),
            winner_id: None,
            winner_name: String::new(),
            error_msg: None,
            logs: vec![format!("Connecting to {}:{}...", target_ip, target_port)],
            server_name: server_name,
            server_ip: format!("{}:{}", target_ip, target_port),
        };

        let state_arc = Arc::new(Mutex::new(initial_snap));
        let state_writer = state_arc.clone();

        thread::spawn(move || {
            if let Err(e) = super::client_loop::run_client_loop(
                target_ip,
                target_port,
                player_name,
                cmd_rx,
                state_writer.clone(),
            ) {
                eprintln!("[CLIENT ERROR] {}", e);
                let mut snap = state_writer.lock().unwrap();
                if snap.error_msg.is_none() {
                    snap.error_msg = Some(format!("{}", e));
                }
                snap.connected = false;
            }
        });

        Ok(Self {
            cmd_tx,
            state_arc,
            active_direction: Direction::None,
        })
    }

    /// Function to update and send the current player movement direction
    pub fn set_direction(&mut self, dir: Direction) {
        if self.active_direction != dir {
            self.active_direction = dir;
            let _ = self.cmd_tx.send(ClientCommand::SendInput(dir));
        }
    }

    /// Function to send an interaction command to the server
    pub fn send_interact(&self) {
        let _ = self.cmd_tx.send(ClientCommand::SendInteract);
    }

    /// Function to send a leave command and disconnect from the server
    pub fn leave(&self) {
        let _ = self.cmd_tx.send(ClientCommand::Leave);
    }

    /// Function to retrieve a clone of the current client state snapshot
    pub fn get_snapshot(&self) -> ClientStateSnapshot {
        self.state_arc.lock().unwrap().clone()
    }
}
