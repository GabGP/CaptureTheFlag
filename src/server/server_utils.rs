use crate::protocol::types::*;

// ============================================================================
// SERVER UTILITIES
// ============================================================================

pub enum ServerCommand {
    StartCountdown,
    StopServer,
}

#[derive(Clone)]
pub struct ServerStateSnapshot {
    pub state: GameState,
    pub game_id: u16,
    pub players: Vec<PlayerState>,
    pub flag_status: FlagStatus,
    pub flag_carrier_id: u16,
    pub flag_x: f32,
    pub flag_y: f32,
    pub tick: u32,
    pub countdown_seconds: u8,
    pub logs: Vec<String>,
    pub server_name: String,
    pub server_ip: String,
    pub winner_id: Option<u16>,
    pub winner_name: String,
}
