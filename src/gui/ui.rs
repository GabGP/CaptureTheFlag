use crate::config::*;
use macroquad::prelude::*;

// ============================================================================
// UI
// ============================================================================

/// Function to draw a GUI panel
pub fn gui_panel(x: f32, y: f32, w: f32, h: f32, title: &str) {
    draw_rectangle(x, y, w, h, Color::from_rgba(20, 26, 42, 230));
    draw_rectangle_lines(x, y, w, h, 2.0, Color::from_rgba(50, 80, 130, 200));

    if !title.is_empty() {
        draw_rectangle(x, y, w, 32.0, Color::from_rgba(30, 42, 68, 255));
        draw_line(
            x,
            y + 32.0,
            x + w,
            y + 32.0,
            1.5,
            Color::from_rgba(60, 100, 160, 255),
        );
        draw_text(
            title,
            x + 12.0,
            y + 22.0,
            FONT_SIZE_HEADER,
            Color::from_rgba(0, 229, 255, 255),
        );
    }
}

/// Function to render a clickable GUI button
pub fn gui_button(x: f32, y: f32, w: f32, h: f32, text: &str, accent: Color) -> bool {
    let mouse_pos = mouse_position();
    let hovered =
        mouse_pos.0 >= x && mouse_pos.0 <= x + w && mouse_pos.1 >= y && mouse_pos.1 <= y + h;
    let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

    let bg_color = if clicked {
        Color::from_rgba(accent.r as u8, accent.g as u8, accent.b as u8, 255)
    } else if hovered {
        Color::from_rgba(
            (accent.r * 255.0 * 0.8) as u8,
            (accent.g * 255.0 * 0.8) as u8,
            (accent.b * 255.0 * 0.8) as u8,
            220,
        )
    } else {
        Color::from_rgba(30, 40, 62, 220)
    };

    let border_color = if hovered {
        accent
    } else {
        Color::from_rgba(60, 90, 140, 255)
    };

    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.0, border_color);

    let font_size = FONT_SIZE_HEADER as u16;
    let dims = measure_text(text, None, font_size, 1.0);
    let text_x = x + (w - dims.width) / 2.0;
    let text_y = y + (h + dims.height) / 2.0 - 2.0;

    let text_color = if clicked || hovered {
        WHITE
    } else {
        Color::from_rgba(220, 230, 245, 255)
    };
    draw_text(text, text_x, text_y, font_size as f32, text_color);

    clicked
}

/// Function to render a text input field
pub fn gui_text_input(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    text: &mut String,
    focused: bool,
    max_len: usize,
) -> bool {
    draw_text(
        label,
        x,
        y - 6.0,
        FONT_SIZE_HEADER,
        Color::from_rgba(160, 180, 210, 255),
    );

    let border_color = if focused {
        Color::from_rgba(0, 229, 255, 255)
    } else {
        Color::from_rgba(60, 80, 120, 255)
    };

    draw_rectangle(x, y, w, h, Color::from_rgba(15, 22, 36, 255));
    draw_rectangle_lines(x, y, w, h, 1.5, border_color);

    if focused {
        while let Some(c) = get_char_pressed() {
            if c >= ' ' && c <= '~' && text.len() < max_len {
                text.push(c);
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            text.pop();
        }
    }

    let display_str = if focused && (get_time() * 2.0) as i32 % 2 == 0 {
        format!("{}|", text)
    } else {
        text.clone()
    };

    draw_text(
        &display_str,
        x + 8.0,
        y + h / 2.0 + 5.0,
        FONT_SIZE_HEADER,
        WHITE,
    );

    let mouse_pos = mouse_position();
    is_mouse_button_pressed(MouseButton::Left)
        && mouse_pos.0 >= x
        && mouse_pos.0 <= x + w
        && mouse_pos.1 >= y
        && mouse_pos.1 <= y + h
}

/// Function to draw logs on the screen
pub fn render_event_logs(x: f32, y: f32, w: f32, h: f32, logs: &[String]) {
    gui_panel(x, y, w, h, "GAME EVENT LOG");
    let line_height = 18.0;
    let start_y = y + 48.0;
    let max_lines = ((h - 55.0) / line_height) as usize;

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
            Color::from_rgba(0, 229, 255, 255)
        } else if line.contains("disconnected") {
            Color::from_rgba(239, 83, 80, 255)
        } else {
            Color::from_rgba(180, 200, 225, 255)
        };
        draw_text(line, x + 12.0, ly, FONT_SIZE_REGULAR, color);
    }
}
