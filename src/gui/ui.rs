use crate::config::*;
use macroquad::prelude::*;

// ============================================================================
// UI
// ============================================================================

/// Function to draw a GUI panel
pub fn gui_panel(x: f32, y: f32, w: f32, h: f32, title: &str) {
    draw_rectangle(x, y, w, h, COLOR_UI_PANEL_BG);
    draw_rectangle_lines(x, y, w, h, UI_PANEL_BORDER_THICKNESS, COLOR_UI_PANEL_BORDER);

    if !title.is_empty() {
        draw_rectangle(x, y, w, UI_PANEL_TITLE_HEIGHT, COLOR_UI_PANEL_TITLE_BG);
        draw_line(
            x,
            y + 32.0,
            x + w,
            y + 32.0,
            UI_PANEL_TITLE_LINE_THICKNESS,
            COLOR_UI_PANEL_TITLE_LINE,
        );
        draw_text(
            title,
            x + UI_PANEL_TITLE_TEXT_OFFSET_X,
            y + UI_PANEL_TITLE_TEXT_OFFSET_Y,
            FONT_SIZE_MEDIUM,
            COLOR_UI_ACCENT_CYAN,
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
            (accent.r * 255.0 * UI_BUTTON_HOVER_DARKEN) as u8,
            (accent.g * 255.0 * UI_BUTTON_HOVER_DARKEN) as u8,
            (accent.b * 255.0 * UI_BUTTON_HOVER_DARKEN) as u8,
            220,
        )
    } else {
        COLOR_UI_BUTTON_BG
    };

    let border_color = if hovered {
        accent
    } else {
        COLOR_UI_BUTTON_BORDER
    };

    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, UI_BUTTON_BORDER_THICKNESS, border_color);

    let font_size = FONT_SIZE_MEDIUM as u16;
    let dims = measure_text(text, None, font_size, 1.0);
    let text_x = x + (w - dims.width) / 2.0;
    let text_y = y + (h + dims.height) / 2.0 + UI_BUTTON_TEXT_OFFSET_Y;

    let text_color = if clicked || hovered {
        WHITE
    } else {
        COLOR_UI_BUTTON_TEXT
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
        y + UI_INPUT_LABEL_OFFSET_Y,
        FONT_SIZE_MEDIUM,
        COLOR_UI_INPUT_LABEL,
    );

    let border_color = if focused {
        COLOR_UI_ACCENT_CYAN
    } else {
        COLOR_UI_INPUT_BORDER
    };

    draw_rectangle(x, y, w, h, COLOR_UI_INPUT_BG);
    draw_rectangle_lines(x, y, w, h, UI_INPUT_BORDER_THICKNESS, border_color);

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
        x + UI_INPUT_TEXT_OFFSET_X,
        y + h / 2.0 + UI_INPUT_TEXT_OFFSET_Y,
        FONT_SIZE_MEDIUM,
        WHITE,
    );

    let mouse_pos = mouse_position();
    is_mouse_button_pressed(MouseButton::Left)
        && mouse_pos.0 >= x
        && mouse_pos.0 <= x + w
        && mouse_pos.1 >= y
        && mouse_pos.1 <= y + h
}
