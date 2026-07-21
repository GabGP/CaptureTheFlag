use serde::{Deserialize, Serialize};

// Each message has a JSON structure terminated by a newline character and encoded in UTF-8.
const PROTOCOL_VERSION: &str = "1.0";

// ============================================================================
// CLIENT MESSAGES
// ============================================================================

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientMessage {
    Join {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        name: String,
    },
    ChangeDirection {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        #[serde(rename = "playerId")]
        player_id: String,
        direction: String,
    },
    Leave {
        #[serde(rename = "protocolVersion")]
        protocol_version: String,
        #[serde(rename = "gameId")]
        game_id: String,
        #[serde(rename = "playerId")]
        player_id: String,
    },
}

impl ClientMessage {
    /// Creates a JOIN message.
    pub fn join(name: &str) -> String {
        let message = Self::Join {
            protocol_version: PROTOCOL_VERSION.to_string(),
            name: name.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a CHANGE_DIRECTION message.
    /// Valid directions are: "UP", "DOWN", "LEFT", "RIGHT".
    pub fn change_direction(game_id: &str, player_id: &str, direction: &str) -> String {
        let message = Self::ChangeDirection {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            player_id: player_id.to_string(),
            direction: direction.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }

    /// Creates a LEAVE message.
    pub fn leave(game_id: &str, player_id: &str) -> String {
        let message = Self::Leave {
            protocol_version: PROTOCOL_VERSION.to_string(),
            game_id: game_id.to_string(),
            player_id: player_id.to_string(),
        };
        return serde_json::to_string(&message).unwrap() + "\n";
    }
}
