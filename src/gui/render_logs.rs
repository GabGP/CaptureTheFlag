use crate::{config::*, gui::ui::gui_panel};
use macroquad::prelude::*;

// ============================================================================
// RENDER LOGS
// ============================================================================

/// Function to draw logs on the screen
pub fn render_event_logs(x: f32, y: f32, w: f32, h: f32, logs: &[String]) {
    gui_panel(x, y, w, h, "GAME EVENT LOG");
    let line_height = UI_LOG_LINE_HEIGHT;
    let start_y = y + UI_LOG_START_OFFSET_Y;
    let max_lines = ((h - UI_LOG_HEIGHT_PADDING) / line_height) as usize;

    let display_logs = if logs.len() > max_lines {
        &logs[logs.len() - max_lines..]
    } else {
        logs
    };

    for (idx, line) in display_logs.iter().enumerate() {
        let ly = start_y + (idx as f32) * line_height;
        let color = if line.contains("WINNER") || line.contains("GAME OVER") {
            GOLD
        } else if line.contains("stole") || line.contains("picked up") {
            COLOR_UI_ACCENT_CYAN
        } else if line.contains("disconnected") {
            COLOR_ERROR
        } else {
            COLOR_LOG_DEFAULT
        };
        draw_text(line, x + UI_LOG_TEXT_OFFSET_X, ly, FONT_SIZE_REGULAR, color);
    }
}
