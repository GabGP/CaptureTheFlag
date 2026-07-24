use crate::{
    config::*,
    server::server_message::{Flag, Obstacle, PlayerState},
};
use macroquad::rand::gen_range;

// ============================================================================
// BOARD & GENERATION LOGIC
// ============================================================================

pub struct Board {
    pub rows: i32,
    pub columns: i32,
    pub obstacles: Vec<Obstacle>,
    pub flag: Flag,
    pub players: Vec<PlayerState>,
}

impl Board {
    /// Generates a new board following the default specification parameters.
    pub fn generate() -> Self {
        let rows = BOARD_ROWS;
        let columns = BOARD_COLUMNS;
        let obstacle_percentage = OBSTACLE_PERCENTAGE;
        let central_area_percentage = CENTRAL_AREA_PERCENTAGE;

        // Calculate the central area boundaries for the flag
        // A 30% area of 20x20 means leaving 35% empty on each side (100 - 30) / 2
        let margin_percent = (1.0 - central_area_percentage) / 2.0;
        let row_margin = (rows as f32 * margin_percent) as i32;
        let col_margin = (columns as f32 * margin_percent) as i32;

        // Generate flag coordinates within the central bounds
        let flag_row = gen_range(row_margin, rows - row_margin);
        let flag_col = gen_range(col_margin, columns - col_margin);

        let flag = Flag {
            row: flag_row,
            column: flag_col,
            status: "AVAILABLE".to_string(),
            carrier_id: None,
        };

        // Calculate and generate obstacles
        let total_cells = rows * columns;
        let num_obstacles = (total_cells as f32 * obstacle_percentage) as i32;
        let mut obstacles: Vec<Obstacle> = Vec::new();

        while obstacles.len() < num_obstacles as usize {
            let r = gen_range(0, rows);
            let c = gen_range(0, columns);

            // Constraint: Obstacles cannot be placed on the flag
            if r == flag_row && c == flag_col {
                continue;
            }

            // Constraint: Avoid placing multiple obstacles on the same cell
            if obstacles.iter().any(|o| o.row == r && o.column == c) {
                continue;
            }

            obstacles.push(Obstacle { row: r, column: c });
        }

        Self {
            rows,
            columns,
            obstacles,
            flag,
            players: Vec::new(),
        }
    }
}
