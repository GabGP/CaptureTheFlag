use macroquad::{color::colors::*, text::draw_text};

// ============================================================================
// LOGS
// ============================================================================

/// Function to draw logs on the screen.
pub fn draw_logs(logs: &[String]) {
    let mut y = 50.0;
    for log in logs.iter().rev().take(15).rev() {
        draw_text(log, 20.0, y, 20.0, WHITE);
        y += 25.0;
    }
}
