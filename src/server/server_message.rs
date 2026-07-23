use serde::{Deserialize, Serialize};

// Each message has a JSON structure terminated by a newline character and encoded in UTF-8.
const PROTOCOL_VERSION: &str = "1.0";

// ============================================================================
// SERVER STRUCTURES
// ============================================================================

#[derive(Deserialize, Serialize, Clone)]
pub struct Obstacle {
    pub row: i32,
    pub column: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Flag {
    pub row: i32,
    pub column: i32,
    pub status: String,
    #[serde(rename = "carrierId")]
    pub carrier_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PlayerState {
    #[serde(rename = "playerId")]
    pub player_id: String,
    pub name: String,
    pub row: i32,
    pub column: i32,
    pub direction: String,
    #[serde(rename = "insideBoard")]
    pub inside_board: bool,
    #[serde(rename = "hasFlag")]
    pub has_flag: bool,
    pub protected: bool,
}

// ============================================================================
// SERVER MESSAGES
// ============================================================================

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerMessage {
    JoinAccepted {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "playerId")]
        player_id: String,
        #[serde(rename = "gameId")]
        game_id: String,
    },
    JoinRejected {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        reason: String,
    },
    GameStarted {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        rows: i32,
        columns: i32,
        #[serde(rename = "movementIntervalMs")]
        movement_interval_ms: i32,
        #[serde(rename = "protectionTimeMs")]
        protection_time_ms: i32,
        obstacles: Vec<Obstacle>,
        flag: Flag,
        players: Vec<PlayerState>,
    },
    GameState {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        tick: i32,
        players: Vec<PlayerState>,
        flag: Flag,
    },
    FlagPickedUp {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        tick: i32,
        #[serde(rename = "playerId")]
        player_id: String,
    },
    FlagStolen {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        tick: i32,
        #[serde(rename = "previousCarrierId")]
        previous_carrier_id: String,
        #[serde(rename = "newCarrierId")]
        new_carrier_id: String,
        #[serde(rename = "protectionTimeMs")]
        protection_time_ms: i32,
    },
    PlayerDisconnected {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        #[serde(rename = "playerId")]
        player_id: String,
    },
    GameOver {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        #[serde(rename = "winnerId")]
        winner_id: String,
        #[serde(rename = "winnerName")]
        winner_name: String,
        reason: String,
    },
    Error {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        code: String,
        description: String,
    },
}

impl ServerMessage {
    /// Creates a JOIN_ACCEPTED message.
    pub fn join_accepted(player_id: &str, game_id: &str) -> String {
        let message = Self::JoinAccepted {
            protocol_version: PROTOCOL_VERSION.to_string(),
            player_id: player_id.to_string(),
            game_id: game_id.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a JOIN_REJECTED message.
    pub fn join_rejected(reason: &str) -> String {
        let message = Self::JoinRejected {
            protocol_version: PROTOCOL_VERSION.to_string(),
            reason: reason.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a GAME_STARTED message.
    pub fn game_started(
        game_id: &str,
        rows: i32,
        columns: i32,
        movement_interval_ms: i32,
        protection_time_ms: i32,
        obstacles: Vec<Obstacle>,
        flag: Flag,
        players: Vec<PlayerState>,
    ) -> String {
        let message = Self::GameStarted {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            rows,
            columns,
            movement_interval_ms,
            protection_time_ms,
            obstacles,
            flag,
            players,
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a GAME_STATE message.
    pub fn game_state(game_id: &str, tick: i32, players: Vec<PlayerState>, flag: Flag) -> String {
        let message = Self::GameState {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            tick,
            players,
            flag,
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a FLAG_PICKED_UP message.
    pub fn flag_picked_up(game_id: &str, tick: i32, player_id: &str) -> String {
        let message = Self::FlagPickedUp {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            tick,
            player_id: player_id.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a FLAG_STOLEN message.
    pub fn flag_stolen(
        game_id: &str,
        tick: i32,
        previous_carrier_id: &str,
        new_carrier_id: &str,
        protection_time_ms: i32,
    ) -> String {
        let message = Self::FlagStolen {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            tick,
            previous_carrier_id: previous_carrier_id.to_string(),
            new_carrier_id: new_carrier_id.to_string(),
            protection_time_ms,
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a PLAYER_DISCONNECTED message.
    pub fn player_disconnected(game_id: &str, player_id: &str) -> String {
        let message = Self::PlayerDisconnected {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            player_id: player_id.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a GAME_OVER message.
    pub fn game_over(game_id: &str, winner_id: &str, winner_name: &str, reason: &str) -> String {
        let message = Self::GameOver {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            winner_id: winner_id.to_string(),
            winner_name: winner_name.to_string(),
            reason: reason.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates an ERROR message.
    pub fn error(code: &str, description: &str) -> String {
        let message = Self::Error {
            protocol_version: PROTOCOL_VERSION.to_string(),
            code: code.to_string(),
            description: description.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }
}
