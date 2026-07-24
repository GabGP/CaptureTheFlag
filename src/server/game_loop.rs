use crate::{config::*, game::board::Board, server::server_utils::*};
use macroquad::time::get_time;
use std::net::TcpStream;

// ============================================================================
// GAME LOOP HELPERS
// ============================================================================

/// Calculates movements and handles collisions for all players on the board
pub fn calculate_movements_and_collisions(board: &mut Board) {
    for player in board.players.iter_mut() {
        let mut next_row = player.row;
        let mut next_col = player.column;

        // Apply directional logic
        match player.direction.as_str() {
            "UP" => next_row -= 1,
            "DOWN" => next_row += 1,
            "LEFT" => next_col -= 1,
            "RIGHT" => next_col += 1,
            _ => {}
        }

        // Boundary Check: Ensure coordinates are within 0..rows and 0..columns
        let is_within_bounds =
            next_row >= 0 && next_row < board.rows && next_col >= 0 && next_col < board.columns;

        // Obstacle Check: Ensure no obstacle exists at the target coordinates
        let hits_obstacle = board
            .obstacles
            .iter()
            .any(|o| o.row == next_row && o.column == next_col);

        // Movement Execution:
        // Players can move if the next tile is valid OR if they are still outside the board trying to enter.
        if (!hits_obstacle && is_within_bounds) || !player.inside_board {
            player.row = next_row;
            player.column = next_col;

            // Once they step into bounds (row >= 0), update their status
            if is_within_bounds && !player.inside_board {
                player.inside_board = true;
            }
        }
    }
}

// ============================================================================
// GAME LOOP
// ============================================================================

/// Executes the game loop logic at fixed intervals
pub fn process_game_tick(
    active_clients: &mut Vec<TcpStream>,
    board: &mut Board,
    last_tick_time: &mut f64,
    tick_counter: &mut i32,
) {
    let current_time = get_time();
    let movement_interval = MOVEMENT_INTERVAL;

    // Only run the logic if the fixed interval has passed since the last tick
    if current_time - *last_tick_time >= movement_interval {
        *last_tick_time = current_time;
        *tick_counter += 1;

        calculate_movements_and_collisions(board);
        broadcast_game_state(active_clients, board, *tick_counter);
    }
}
