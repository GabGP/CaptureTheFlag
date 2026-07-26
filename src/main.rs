mod app;
mod client;
mod config;
mod gui;
mod protocol;
mod server;

use app::AppRunner;
use macroquad::prelude::*;

// ============================================================================
// MAIN APPLICATION
// ============================================================================

fn window_conf() -> Conf {
    Conf {
        window_title: "Capture The Flag".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let custom_font = load_ttf_font("./assets/fonts/JetBrainsMono-Bold.ttf")
        .await
        .unwrap_or_else(|_| {
            panic!("Failed to load font from assets/fonts/JetBrainsMono-Regular.ttf");
        });

    set_default_font(custom_font);

    let mut app = AppRunner::new();

    loop {
        app.update().await;
        next_frame().await;
    }
}
