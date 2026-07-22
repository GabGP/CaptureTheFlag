use macroquad::{color::colors::*, text::draw_text};

// ============================================================================
// LOGS CONFIGURATION
// ============================================================================

const MAX_LOGS: usize = 20;

// Font size and spacing
const FONT_SIZE: f32 = 20.0;
const SPACING: f32 = 5.0;
const LINE_SPACING: f32 = FONT_SIZE + SPACING;

// Logs starting position
const LOG_START_X: f32 = 20.0;
const LOG_START_Y: f32 = 50.0;

// ============================================================================
// LOGS
// ============================================================================

/// Function to draw logs on the screen.
pub fn draw_logs(logs: &[String]) {
    let mut y = LOG_START_Y;
    for log in logs.iter().rev().take(MAX_LOGS).rev() {
        draw_text(log, LOG_START_X, y, FONT_SIZE, WHITE);
        y += LINE_SPACING;
    }
}
