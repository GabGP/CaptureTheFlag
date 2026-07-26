use crate::protocol::{types::*, utils};
use std::io;

pub const PROTOCOL_VERSION: u8 = 3;

// ============================================================================
// PROTOCOL DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    DiscoverRequest,
    DiscoverResponse {
        game_id: u16,
        server_name: String,
        tcp_port: u16,
        state: GameState,
        player_count: u16,
        maximum_players: u16,
    },
    Join {
        name: String,
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
    JoinAccepted {
        player_id: u16,
        game_id: u16,
    },
    JoinRejected {
        reason: JoinRejectReason,
    },
    LobbyState {
        state: GameState,
        players: Vec<LobbyPlayer>,
    },
    GameCountdown {
        seconds_remaining: u8,
    },
    GameStarted {
        config: GameConfig,
        flag_status: FlagStatus,
        flag_carrier_id: u16,
        flag_x: f32,
        flag_y: f32,
        players: Vec<PlayerState>,
    },
    GameStateMsg {
        tick: u32,
        flag_status: FlagStatus,
        flag_carrier_id: u16,
        flag_x: f32,
        flag_y: f32,
        players: Vec<PlayerState>,
    },
    FlagPickedUp {
        tick: u32,
        player_id: u16,
    },
    FlagStolen {
        tick: u32,
        previous_carrier_id: u16,
        new_carrier_id: u16,
    },
    PlayerDisconnected {
        player_id: u16,
    },
    GameOver {
        winner_id: u16,
        winner_name: String,
        reason: GameOverReason,
    },
    ErrorMsg {
        code: ErrorCode,
        description: String,
    },
}

impl Message {
    // ============================================================================
    // MESSAGE SERIALIZATION
    // ============================================================================

    /// Function to pack a structured message into a raw byte array for sending over the network
    pub fn serialize(&self) -> Vec<u8> {
        let mut w = utils::ByteWriter::new();
        match self {
            Message::DiscoverRequest => {
                w.write_u8(0x01);
                w.write_u8(PROTOCOL_VERSION);
            }
            Message::DiscoverResponse {
                game_id,
                server_name,
                tcp_port,
                state,
                player_count,
                maximum_players,
            } => {
                w.write_u8(0x02);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*game_id);
                w.write_str(server_name);
                w.write_u16(*tcp_port);
                w.write_u8(*state as u8);
                w.write_u16(*player_count);
                w.write_u16(*maximum_players);
            }
            Message::Join { name } => {
                w.write_u8(0x10);
                w.write_u8(PROTOCOL_VERSION);
                w.write_str(name);
            }
            Message::Input {
                player_id,
                direction,
            } => {
                w.write_u8(0x11);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*player_id);
                w.write_u8(*direction as u8);
            }
            Message::Interact { player_id } => {
                w.write_u8(0x12);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*player_id);
            }
            Message::Leave { player_id } => {
                w.write_u8(0x13);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*player_id);
            }
            Message::JoinAccepted { player_id, game_id } => {
                w.write_u8(0x20);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*player_id);
                w.write_u16(*game_id);
            }
            Message::JoinRejected { reason } => {
                w.write_u8(0x21);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u8(*reason as u8);
            }
            Message::LobbyState { state, players } => {
                w.write_u8(0x22);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u8(*state as u8);
                w.write_u8(players.len() as u8);
                for p in players {
                    w.write_u16(p.player_id);
                    w.write_str(&p.name);
                }
            }
            Message::GameCountdown { seconds_remaining } => {
                w.write_u8(0x23);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u8(*seconds_remaining);
            }
            Message::GameStarted {
                config,
                flag_status,
                flag_carrier_id,
                flag_x,
                flag_y,
                players,
            } => {
                w.write_u8(0x24);
                w.write_u8(PROTOCOL_VERSION);
                w.write_i32(utils::float_to_i32(config.map_size));
                w.write_i32(utils::float_to_i32(config.circle_radius));
                w.write_i32(utils::float_to_i32(config.player_radius));
                w.write_i32(utils::float_to_i32(config.player_speed));
                w.write_i32(utils::float_to_i32(config.interaction_radius));
                w.write_u16(config.tick_interval_ms);
                w.write_u8(*flag_status as u8);
                w.write_u16(*flag_carrier_id);
                w.write_i32(utils::float_to_i32(*flag_x));
                w.write_i32(utils::float_to_i32(*flag_y));
                w.write_u8(players.len() as u8);
                for p in players {
                    w.write_u16(p.player_id);
                    w.write_str(&p.name);
                    w.write_i32(utils::float_to_i32(p.x));
                    w.write_i32(utils::float_to_i32(p.y));
                    w.write_u8(p.direction as u8);
                    w.write_bool(p.has_flag);
                }
            }
            Message::GameStateMsg {
                tick,
                flag_status,
                flag_carrier_id,
                flag_x,
                flag_y,
                players,
            } => {
                w.write_u8(0x25);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u32(*tick);
                w.write_u8(*flag_status as u8);
                w.write_u16(*flag_carrier_id);
                w.write_i32(utils::float_to_i32(*flag_x));
                w.write_i32(utils::float_to_i32(*flag_y));
                w.write_u8(players.len() as u8);
                for p in players {
                    w.write_u16(p.player_id);
                    w.write_i32(utils::float_to_i32(p.x));
                    w.write_i32(utils::float_to_i32(p.y));
                    w.write_u8(p.direction as u8);
                    w.write_bool(p.has_flag);
                }
            }
            Message::FlagPickedUp { tick, player_id } => {
                w.write_u8(0x26);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u32(*tick);
                w.write_u16(*player_id);
            }
            Message::FlagStolen {
                tick,
                previous_carrier_id,
                new_carrier_id,
            } => {
                w.write_u8(0x27);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u32(*tick);
                w.write_u16(*previous_carrier_id);
                w.write_u16(*new_carrier_id);
            }
            Message::PlayerDisconnected { player_id } => {
                w.write_u8(0x28);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*player_id);
            }
            Message::GameOver {
                winner_id,
                winner_name,
                reason,
            } => {
                w.write_u8(0x29);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u16(*winner_id);
                w.write_str(winner_name);
                w.write_u8(*reason as u8);
            }
            Message::ErrorMsg { code, description } => {
                w.write_u8(0x2A);
                w.write_u8(PROTOCOL_VERSION);
                w.write_u8(*code as u8);
                w.write_str(description);
            }
        }
        w.into_bytes()
    }

    // ============================================================================
    // MESSAGE DESERIALIZATION
    // ============================================================================

    /// Function to unpack a raw network byte array and convert it back into a readable message
    pub fn deserialize(buf: &[u8]) -> Result<Self, io::Error> {
        let mut r = utils::ByteReader::new(buf);
        let msg_type = r.read_u8()?;
        let version = r.read_u8()?;

        if version != PROTOCOL_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported protocol version: {}", version),
            ));
        }

        match msg_type {
            0x01 => Ok(Message::DiscoverRequest),
            0x02 => {
                let game_id = r.read_u16()?;
                let server_name = r.read_str()?;
                let tcp_port = r.read_u16()?;
                let state_val = r.read_u8()?;
                let state = GameState::from_u8(state_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid GameState")
                })?;
                let player_count = r.read_u16()?;
                let maximum_players = r.read_u16()?;
                Ok(Message::DiscoverResponse {
                    game_id,
                    server_name,
                    tcp_port,
                    state,
                    player_count,
                    maximum_players,
                })
            }
            0x10 => {
                let name = r.read_str()?;
                Ok(Message::Join { name })
            }
            0x11 => {
                let player_id = r.read_u16()?;
                let dir_val = r.read_u8()?;
                let direction = Direction::from_u8(dir_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid Direction")
                })?;
                Ok(Message::Input {
                    player_id,
                    direction,
                })
            }
            0x12 => {
                let player_id = r.read_u16()?;
                Ok(Message::Interact { player_id })
            }
            0x13 => {
                let player_id = r.read_u16()?;
                Ok(Message::Leave { player_id })
            }
            0x20 => {
                let player_id = r.read_u16()?;
                let game_id = r.read_u16()?;
                Ok(Message::JoinAccepted { player_id, game_id })
            }
            0x21 => {
                let reason_val = r.read_u8()?;
                let reason = JoinRejectReason::from_u8(reason_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid JoinRejectReason")
                })?;
                Ok(Message::JoinRejected { reason })
            }
            0x22 => {
                let state_val = r.read_u8()?;
                let state = GameState::from_u8(state_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid GameState")
                })?;
                let count = r.read_u8()?;
                let mut players = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let player_id = r.read_u16()?;
                    let name = r.read_str()?;
                    players.push(LobbyPlayer { player_id, name });
                }
                Ok(Message::LobbyState { state, players })
            }
            0x23 => {
                let seconds_remaining = r.read_u8()?;
                Ok(Message::GameCountdown { seconds_remaining })
            }
            0x24 => {
                let map_size = utils::i32_to_float(r.read_i32()?);
                let circle_radius = utils::i32_to_float(r.read_i32()?);
                let player_radius = utils::i32_to_float(r.read_i32()?);
                let player_speed = utils::i32_to_float(r.read_i32()?);
                let interaction_radius = utils::i32_to_float(r.read_i32()?);
                let tick_interval_ms = r.read_u16()?;
                let flag_status_val = r.read_u8()?;
                let flag_status = FlagStatus::from_u8(flag_status_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid FlagStatus")
                })?;
                let flag_carrier_id = r.read_u16()?;
                let flag_x = utils::i32_to_float(r.read_i32()?);
                let flag_y = utils::i32_to_float(r.read_i32()?);
                let count = r.read_u8()?;
                let mut players = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let player_id = r.read_u16()?;
                    let name = r.read_str()?;
                    let x = utils::i32_to_float(r.read_i32()?);
                    let y = utils::i32_to_float(r.read_i32()?);
                    let dir_val = r.read_u8()?;
                    let direction = Direction::from_u8(dir_val).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid Direction")
                    })?;
                    let has_flag = r.read_bool()?;
                    players.push(PlayerState {
                        player_id,
                        name,
                        x,
                        y,
                        direction,
                        has_flag,
                    });
                }
                let config = GameConfig {
                    map_size,
                    circle_radius,
                    player_radius,
                    spawn_margin: 80.0,
                    player_speed,
                    interaction_radius,
                    tick_interval_ms,
                    countdown_seconds: 5,
                    maximum_players: 100,
                    server_port: 5000,
                    discovery_port: 5001,
                };
                Ok(Message::GameStarted {
                    config,
                    flag_status,
                    flag_carrier_id,
                    flag_x,
                    flag_y,
                    players,
                })
            }
            0x25 => {
                let tick = r.read_u32()?;
                let flag_status_val = r.read_u8()?;
                let flag_status = FlagStatus::from_u8(flag_status_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid FlagStatus")
                })?;
                let flag_carrier_id = r.read_u16()?;
                let flag_x = utils::i32_to_float(r.read_i32()?);
                let flag_y = utils::i32_to_float(r.read_i32()?);
                let count = r.read_u8()?;
                let mut players = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let player_id = r.read_u16()?;
                    let x = utils::i32_to_float(r.read_i32()?);
                    let y = utils::i32_to_float(r.read_i32()?);
                    let dir_val = r.read_u8()?;
                    let direction = Direction::from_u8(dir_val).ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Invalid Direction")
                    })?;
                    let has_flag = r.read_bool()?;
                    players.push(PlayerState {
                        player_id,
                        name: String::new(),
                        x,
                        y,
                        direction,
                        has_flag,
                    });
                }
                Ok(Message::GameStateMsg {
                    tick,
                    flag_status,
                    flag_carrier_id,
                    flag_x,
                    flag_y,
                    players,
                })
            }
            0x26 => {
                let tick = r.read_u32()?;
                let player_id = r.read_u16()?;
                Ok(Message::FlagPickedUp { tick, player_id })
            }
            0x27 => {
                let tick = r.read_u32()?;
                let previous_carrier_id = r.read_u16()?;
                let new_carrier_id = r.read_u16()?;
                Ok(Message::FlagStolen {
                    tick,
                    previous_carrier_id,
                    new_carrier_id,
                })
            }
            0x28 => {
                let player_id = r.read_u16()?;
                Ok(Message::PlayerDisconnected { player_id })
            }
            0x29 => {
                let winner_id = r.read_u16()?;
                let winner_name = r.read_str()?;
                let reason_val = r.read_u8()?;
                let reason = GameOverReason::from_u8(reason_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid GameOverReason")
                })?;
                Ok(Message::GameOver {
                    winner_id,
                    winner_name,
                    reason,
                })
            }
            0x2A => {
                let code_val = r.read_u8()?;
                let code = ErrorCode::from_u8(code_val).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid ErrorCode")
                })?;
                let description = r.read_str()?;
                Ok(Message::ErrorMsg { code, description })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown message type: 0x{:02X}", msg_type),
            )),
        }
    }
}
