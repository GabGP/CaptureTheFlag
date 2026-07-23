// ============================================================================
// BOARD CONFIGURATION
// ============================================================================

pub const BOARD_ROWS: i32 = 20;
pub const BOARD_COLUMNS: i32 = 20;
pub const OBSTACLE_PERCENTAGE: f32 = 0.10; // 10% of total cells
pub const CENTRAL_AREA_PERCENTAGE: f32 = 0.30; // 30% center area

// ============================================================================
// GAME SERVER CONFIGURATION
// ============================================================================

pub const MOVEMENT_INTERVAL_MS: i32 = 200;
pub const MOVEMENT_INTERVAL: f64 = (MOVEMENT_INTERVAL_MS as f32 / 1000.0) as f64;
pub const PROTECTION_TIME_MS: i32 = 1000;

// ============================================================================
// LOGS CONFIGURATION
// ============================================================================

pub const LOG_MAX: usize = 20;

// Font size and spacing
pub const LOG_FONT_SIZE: f32 = 20.0;
pub const LOG_SPACING: f32 = 5.0;
pub const LOG_LINE_SPACING: f32 = LOG_FONT_SIZE + LOG_SPACING;

// Logs starting position
pub const LOG_START_X: f32 = 20.0;
pub const LOG_START_Y: f32 = 50.0;
