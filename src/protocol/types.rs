#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    None = 0x00,
    Up = 0x01,
    Down = 0x02,
    Left = 0x03,
    Right = 0x04,
}

impl Direction {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x00 => Some(Direction::None),
            0x01 => Some(Direction::Up),
            0x02 => Some(Direction::Down),
            0x03 => Some(Direction::Left),
            0x04 => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagStatus {
    Available = 0x01,
    Carried = 0x02,
    Dropped = 0x03,
    Outside = 0x04,
}

impl FlagStatus {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(FlagStatus::Available),
            0x02 => Some(FlagStatus::Carried),
            0x03 => Some(FlagStatus::Dropped),
            0x04 => Some(FlagStatus::Outside),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameState {
    Waiting = 0x01,
    Starting = 0x02,
    Running = 0x03,
    Finished = 0x04,
    Cancelled = 0x05,
}

impl GameState {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(GameState::Waiting),
            0x02 => Some(GameState::Starting),
            0x03 => Some(GameState::Running),
            0x04 => Some(GameState::Finished),
            0x05 => Some(GameState::Cancelled),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum JoinRejectReason {
    GameAlreadyStarted = 0x01,
    GameFull = 0x02,
    InvalidName = 0x03,
    UnsupportedProtocolVersion = 0x04,
}

impl JoinRejectReason {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(JoinRejectReason::GameAlreadyStarted),
            0x02 => Some(JoinRejectReason::GameFull),
            0x03 => Some(JoinRejectReason::InvalidName),
            0x04 => Some(JoinRejectReason::UnsupportedProtocolVersion),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameOverReason {
    ExitedCircleWithFlag = 0x01,
}

impl GameOverReason {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(GameOverReason::ExitedCircleWithFlag),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ErrorCode {
    InvalidMessage = 0x01,
    InvalidEncoding = 0x02,
    InvalidInput = 0x03,
    UnknownPlayer = 0x04,
    GameNotStarted = 0x05,
    GameAlreadyStarted = 0x06,
    GameFinished = 0x07,
    UnsupportedProtocolVersion = 0x08,
}

impl ErrorCode {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(ErrorCode::InvalidMessage),
            0x02 => Some(ErrorCode::InvalidEncoding),
            0x03 => Some(ErrorCode::InvalidInput),
            0x04 => Some(ErrorCode::UnknownPlayer),
            0x05 => Some(ErrorCode::GameNotStarted),
            0x06 => Some(ErrorCode::GameAlreadyStarted),
            0x07 => Some(ErrorCode::GameFinished),
            0x08 => Some(ErrorCode::UnsupportedProtocolVersion),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    pub map_size: f32,
    pub circle_radius: f32,
    pub player_radius: f32,
    pub spawn_margin: f32,
    pub player_speed: f32,
    pub interaction_radius: f32,
    pub tick_interval_ms: u16,
    pub countdown_seconds: u8,
    pub maximum_players: u16,
    pub server_port: u16,
    pub discovery_port: u16,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            map_size: 2000.0,
            circle_radius: 500.0,
            player_radius: 15.0,
            spawn_margin: 80.0,
            player_speed: 220.0,
            interaction_radius: 60.0,
            tick_interval_ms: 50,
            countdown_seconds: 5,
            maximum_players: 100,
            server_port: 5000,
            discovery_port: 5001,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LobbyPlayer {
    pub player_id: u16,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerState {
    pub player_id: u16,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub has_flag: bool,
}

#[derive(Debug, Clone)]
pub struct DiscoveredServer {
    pub ip: String,
    pub game_id: u16,
    pub server_name: String,
    pub tcp_port: u16,
    pub state: GameState,
    pub player_count: u16,
    pub max_players: u16,
    pub last_seen: std::time::Instant,
}
