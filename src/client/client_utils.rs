use crate::protocol::types::*;
use std::collections::HashMap;

// ============================================================================
// CLIENT UTILITIES
// ============================================================================

#[derive(Clone)]
pub struct ClientStateSnapshot {
    pub connected: bool,
    pub player_id: u16,
    pub game_id: u16,
    pub game_state: GameState,
    pub lobby_players: Vec<LobbyPlayer>,
    pub countdown_seconds: u8,
    pub config: GameConfig,
    pub flag_status: FlagStatus,
    pub flag_carrier_id: u16,
    pub flag_x: f32,
    pub flag_y: f32,
    pub tick: u32,
    pub players: Vec<PlayerState>,
    pub player_names: HashMap<u16, String>,
    pub winner_id: Option<u16>,
    pub winner_name: String,
    pub error_msg: Option<String>,
    pub logs: Vec<String>,
    pub server_name: String,
    pub server_ip: String,
}

pub enum ClientCommand {
    SendInput(Direction),
    SendInteract,
    Leave,
}
