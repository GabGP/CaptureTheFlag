use crate::config::*;
use macroquad::{color::colors::*, text::draw_text};

// ============================================================================
// LOGS
// ============================================================================

/// Function to draw logs on the screen.
pub fn draw_logs(logs: &[String]) {
    let mut y = LOG_START_Y;
    for log in logs.iter().rev().take(LOG_MAX).rev() {
        draw_text(log, LOG_START_X, y, LOG_FONT_SIZE, WHITE);
        y += LOG_LINE_SPACING;
    }
}
