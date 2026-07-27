use crate::{
    config::*,
    gui::ui::{gui_button, gui_panel},
};
use macroquad::prelude::*;
use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

/// Renders the shrinking countdown number overlay
pub fn render_countdown_overlay(countdown_seconds: u8, time: f32, sw: f32, sh: f32) {
    // Statics scoped strictly to this function
    static LAST_COUNTDOWN: AtomicU8 = AtomicU8::new(255);
    static CHANGE_TIME_BITS: AtomicU32 = AtomicU32::new(0);

    let last_sec = LAST_COUNTDOWN.load(Ordering::Relaxed);
    if countdown_seconds != last_sec {
        LAST_COUNTDOWN.store(countdown_seconds, Ordering::Relaxed);
        CHANGE_TIME_BITS.store(time.to_bits(), Ordering::Relaxed);
    }

    let change_time = f32::from_bits(CHANGE_TIME_BITS.load(Ordering::Relaxed));
    let elapsed = (time - change_time).clamp(0.0, 1.0);

    let shrink = 1.0 - elapsed;
    let ease = shrink * shrink * shrink;
    let pulse_size = 90.0 + (ease * 70.0);

    let alpha = shrink.clamp(0.0, 1.0);
    let main_color = Color::new(GOLD.r, GOLD.g, GOLD.b, alpha);
    let border_color = Color::new(0.0, 0.0, 0.0, alpha);

    let text = format!("{}", countdown_seconds);
    let dims = measure_text(&text, None, pulse_size as u16, 1.0);
    let text_x = sw / 2.0 - (dims.width / 2.0);
    let text_y = sh / 2.0 + (dims.height / 3.0);
    let border_offset = (pulse_size * 0.035).clamp(2.0, 5.0);

    for dx in [-border_offset, 0.0, border_offset] {
        for dy in [-border_offset, 0.0, border_offset] {
            if dx != 0.0 || dy != 0.0 {
                draw_text(&text, text_x + dx, text_y + dy, pulse_size, border_color);
            }
        }
    }
    draw_text(&text, text_x, text_y, pulse_size, main_color);
}

/// Renders the "GO!" burst for the first 15 ticks
pub fn render_go_burst(tick: u32, sw: f32, sh: f32) {
    let progress = tick as f32 / 15.0; // 0.0 -> 1.0
    let shrink = 1.0 - progress;
    let ease = shrink * shrink;
    let pulse_size = 110.0 + (ease * 90.0);

    let alpha = shrink.clamp(0.0, 1.0);
    let main_color = Color::new(GOLD.r, GOLD.g, GOLD.b, alpha);
    let border_color = Color::new(0.0, 0.0, 0.0, alpha);

    let text = "GO!";
    let dims = measure_text(text, None, pulse_size as u16, 1.0);
    let text_x = sw / 2.0 - (dims.width / 2.0);
    let text_y = sh / 2.0 + (dims.height / 3.0);
    let border_offset = (pulse_size * 0.035).clamp(2.0, 5.0);

    for dx in [-border_offset, 0.0, border_offset] {
        for dy in [-border_offset, 0.0, border_offset] {
            if dx != 0.0 || dy != 0.0 {
                draw_text(text, text_x + dx, text_y + dy, pulse_size, border_color);
            }
        }
    }
    draw_text(text, text_x, text_y, pulse_size, main_color);
}

/// Renders the Game Over panel. Returns `true` if the Main Menu button is clicked.
pub fn render_game_over_overlay(winner_name: &str, sw: f32, sh: f32) -> bool {
    draw_rectangle(0.0, 0.0, sw, sh, Color::from_rgba(0, 0, 0, 180));
    gui_panel(
        sw / 2.0 - 250.0,
        sh / 2.0 - 120.0,
        500.0,
        240.0,
        "[*] GAME OVER [*]",
    );

    draw_text(
        &format!("WINNER: {}!", winner_name),
        sw / 2.0 - 150.0,
        sh / 2.0 - 30.0,
        FONT_SIZE_LARGE,
        GOLD,
    );

    draw_text(
        "The winner successfully carried the flag out of the central circle!",
        sw / 2.0 - 210.0,
        sh / 2.0 + 10.0,
        FONT_SIZE_TINY,
        WHITE,
    );

    // Returns true if the user clicks this button
    gui_button(
        sw / 2.0 - 100.0,
        sh / 2.0 + 50.0,
        200.0,
        45.0,
        "MAIN MENU",
        Color::from_rgba(0, 200, 100, 255),
    )
}