use crate::protocol::protocol::Message;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::file_store::{LoggerHandle, format_timestamp};

// ============================================================================
// MESSAGE LOGGER & FORMATTER
// ============================================================================

#[derive(Clone, Copy)]
pub enum LogDirection {
    Sent,
    Received,
}

/// Formats a network message line with timestamps, direction, and identifiers
fn format_message_line(
    direction: LogDirection,
    address: &str,
    side: &str,
    message_type: &str,
    payload: &str,
) -> String {
    let marker = match direction {
        LogDirection::Sent => ">",
        LogDirection::Received => "<",
    };
    format!(
        "{} [{}] <{}> {} {}: {}",
        marker,
        format_timestamp(),
        side,
        address,
        message_type,
        payload
    )
}

// ============================================================================
// GLOBAL STATE & CACHING
// ============================================================================

static LOGGER: OnceLock<LoggerHandle> = OnceLock::new();
static PLAYER_NAMES: OnceLock<Mutex<HashMap<String, HashMap<u16, String>>>> = OnceLock::new();

/// Retrieves or initializes the global file logger handle
fn logger() -> &'static LoggerHandle {
    LOGGER.get_or_init(|| LoggerHandle::new().expect("logger init failed"))
}

/// Retrieves or initializes the global cache for player names
fn player_name_store() -> &'static Mutex<HashMap<String, HashMap<u16, String>>> {
    PLAYER_NAMES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Intercepts lobby and start messages to cache player ID to name mappings
fn remember_player_names(side: &str, message: &Message) {
    match message {
        Message::LobbyState { players, .. } => {
            let mut store = player_name_store().lock().unwrap();
            let side_map = store.entry(side.to_string()).or_default();
            for player in players {
                side_map.insert(player.player_id, player.name.clone());
            }
        }
        Message::GameStarted { players, .. } => {
            let mut store = player_name_store().lock().unwrap();
            let side_map = store.entry(side.to_string()).or_default();
            for player in players {
                side_map.insert(player.player_id, player.name.clone());
            }
        }
        _ => {}
    }
}

/// Attempts to look up a player's name by their ID from the cache
fn resolve_player_name(side: &str, player_id: u16) -> Option<String> {
    let store = player_name_store().lock().unwrap();
    store
        .get(side)
        .and_then(|players| players.get(&player_id))
        .cloned()
}

// ============================================================================
// PUBLIC LOGGING API
// ============================================================================

/// Explicitly triggers initialization of the global logger
pub fn init_logger() {
    let _ = logger();
}

/// Logs a message designated for the client side
pub fn log_client_message(
    address: &str,
    direction: LogDirection,
    message_type: &str,
    payload: &str,
) {
    let line = format_message_line(direction, address, "client", message_type, payload);
    println!("{}", line);
    let _ = logger().append("client", &line);
}

/// Logs a message designated for the server side
pub fn log_server_message(
    address: &str,
    direction: LogDirection,
    message_type: &str,
    payload: &str,
) {
    let line = format_message_line(direction, address, "server", message_type, payload);
    println!("{}", line);
    let _ = logger().append("server", &line);
}

/// High-level generic logger that caches names, serializes payloads, and routes by side
pub fn log_message(address: &str, side: &str, direction: LogDirection, message: &Message) {
    remember_player_names(side, message);
    let payload = format_message_payload(message, side);
    if side == "client" {
        log_client_message(address, direction, message_type_name(message), &payload);
    } else {
        log_server_message(address, direction, message_type_name(message), &payload);
    }
}

// ============================================================================
// MESSAGE SERIALIZATION
// ============================================================================

/// Returns the string representation of the message variant
fn message_type_name(message: &Message) -> &'static str {
    match message {
        Message::DiscoverRequest => "DiscoverRequest",
        Message::DiscoverResponse { .. } => "DiscoverResponse",
        Message::Join { .. } => "Join",
        Message::Input { .. } => "Input",
        Message::Interact { .. } => "Interact",
        Message::Leave { .. } => "Leave",
        Message::JoinAccepted { .. } => "JoinAccepted",
        Message::JoinRejected { .. } => "JoinRejected",
        Message::LobbyState { .. } => "LobbyState",
        Message::GameCountdown { .. } => "GameCountdown",
        Message::GameStarted { .. } => "GameStarted",
        Message::GameStateMsg { .. } => "GameStateMsg",
        Message::FlagPickedUp { .. } => "FlagPickedUp",
        Message::FlagStolen { .. } => "FlagStolen",
        Message::PlayerDisconnected { .. } => "PlayerDisconnected",
        Message::GameOver { .. } => "GameOver",
        Message::ErrorMsg { .. } => "ErrorMsg",
    }
}

/// Formats the inner data of a message variant into a readable log string
fn format_message_payload(message: &Message, side: &str) -> String {
    match message {
        Message::DiscoverRequest => "DiscoverRequest".to_string(),
        Message::DiscoverResponse {
            game_id,
            server_name,
            tcp_port,
            state,
            player_count,
            maximum_players,
        } => {
            format!(
                "game_id={}, server_name={}, tcp_port={}, state={:?}, player_count={}, maximum_players={}",
                game_id, server_name, tcp_port, state, player_count, maximum_players
            )
        }
        Message::Join { name } => format!("name={}", name),
        Message::Input {
            player_id,
            direction,
        } => format!("player_id={}, direction={:?}", player_id, direction),
        Message::Interact { player_id } => format!("player_id={}", player_id),
        Message::Leave { player_id } => format!("player_id={}", player_id),
        Message::JoinAccepted { player_id, game_id } => {
            format!("player_id={}, game_id={}", player_id, game_id)
        }
        Message::JoinRejected { reason } => format!("reason={:?}", reason),
        Message::LobbyState { state, players } => {
            format!("state={:?}, players={:?}", state, players)
        }
        Message::GameCountdown { seconds_remaining } => {
            format!("seconds_remaining={}", seconds_remaining)
        }
        Message::GameStarted { config, .. } => format!("config={:?}", config),
        Message::GameStateMsg {
            tick,
            flag_status,
            flag_carrier_id,
            flag_x,
            flag_y,
            players,
        } => {
            let formatted_players = players
                .iter()
                .map(|p| {
                    let name = resolve_player_name(side, p.player_id);
                    if let Some(name) = name {
                        format!(
                            "player_id={}, name={}, x={}, y={}, direction={:?}, has_flag={}",
                            p.player_id, name, p.x, p.y, p.direction, p.has_flag
                        )
                    } else {
                        format!(
                            "player_id={}, name=, x={}, y={}, direction={:?}, has_flag={}",
                            p.player_id, p.x, p.y, p.direction, p.has_flag
                        )
                    }
                })
                .collect::<Vec<_>>();
            format!(
                "tick={}, flag_status={:?}, flag_carrier_id={}, flag_x={}, flag_y={}, players=[{}]",
                tick,
                flag_status,
                flag_carrier_id,
                flag_x,
                flag_y,
                formatted_players.join(", ")
            )
        }
        Message::FlagPickedUp { tick, player_id } => {
            format!("tick={}, player_id={}", tick, player_id)
        }
        Message::FlagStolen {
            tick,
            previous_carrier_id,
            new_carrier_id,
        } => format!(
            "tick={}, previous_carrier_id={}, new_carrier_id={}",
            tick, previous_carrier_id, new_carrier_id
        ),
        Message::PlayerDisconnected { player_id } => format!("player_id={}", player_id),
        Message::GameOver {
            winner_id,
            winner_name,
            reason,
        } => format!(
            "winner_id={}, winner_name={}, reason={:?}",
            winner_id, winner_name, reason
        ),
        Message::ErrorMsg { code, description } => {
            format!("code={:?}, description={}", code, description)
        }
    }
}
