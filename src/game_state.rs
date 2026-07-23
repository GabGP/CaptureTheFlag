// ============================================================================
// GAME STATE
// ============================================================================

/// Represents the official states of the game session.
#[derive(Debug, PartialEq, Clone)]
pub enum GameState {
    Waiting,
    Starting,
    Running,
    Finished,
    Cancelled,
}

