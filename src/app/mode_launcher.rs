use crate::{
    client::client::GameClient,
    config::*,
    gui::ui::{gui_button, gui_panel, gui_text_input},
    protocol::types::*,
    server::server::GameServer,
};
use macroquad::prelude::*;

use super::runner::{AppMode, AppRunner};

// ============================================================================
// MODE LAUNCHER
// ============================================================================

/// Function to handle launcher UI updates, server discovery scanning, and connection routing
pub fn update(app: &mut AppRunner, time: f32) -> Option<AppMode> {
    clear_background(Color::from_rgba(12, 16, 26, 255));

    if let Some(ref sc) = app.scanner {
        if time as f64 - app.last_scan_time > 2.0 {
            app.last_scan_time = time as f64;
            sc.scan(5001);
        }
    }

    let sw = screen_width();
    draw_text(
        "CAPTURE THE FLAG",
        sw / 2.0 - 190.0,
        70.0,
        FONT_SIZE_TITLE,
        COLOR_UI_ACCENT_CYAN,
    );
    draw_text(
        "PRFC Version 3.0.0 Binary Protocol - Rust Sockets & Macroquad",
        sw / 2.0 - 240.0,
        100.0,
        FONT_SIZE_REGULAR,
        Color::from_rgba(160, 190, 220, 255),
    );

    let p1_x = 60.0;
    let p1_y = 130.0;
    let p1_w = 480.0;
    let p1_h = 530.0;
    gui_panel(p1_x, p1_y, p1_w, p1_h, "LAUNCHER SETTINGS");

    if gui_text_input(
        p1_x + 30.0,
        p1_y + 70.0,
        420.0,
        36.0,
        "PLAYER NAME",
        &mut app.player_name_input,
        app.focus_field == 1,
        20,
    ) {
        app.focus_field = 1;
    }
    if gui_text_input(
        p1_x + 30.0,
        p1_y + 140.0,
        420.0,
        36.0,
        "HOST SERVER NAME",
        &mut app.server_name_input,
        app.focus_field == 2,
        25,
    ) {
        app.focus_field = 2;
    }
    if gui_text_input(
        p1_x + 30.0,
        p1_y + 210.0,
        270.0,
        36.0,
        "SERVER IP ADDRESS",
        &mut app.target_ip_input,
        app.focus_field == 3,
        30,
    ) {
        app.focus_field = 3;
    }
    if gui_text_input(
        p1_x + 310.0,
        p1_y + 210.0,
        140.0,
        36.0,
        "TCP PORT",
        &mut app.target_port_input,
        app.focus_field == 4,
        6,
    ) {
        app.focus_field = 4;
    }

    if gui_button(
        p1_x + 30.0,
        p1_y + 280.0,
        420.0,
        50.0,
        "[+] HOST NEW SERVER (TCP 5000 / UDP 5001)",
        Color::from_rgba(30, 45, 65, 255),
    ) {
        let port: u16 = app.target_port_input.parse().unwrap_or(5000);
        let mut config = GameConfig::default();
        config.server_port = port;
        let name = if app.server_name_input.trim().is_empty() {
            "Rust CTF Server".to_string()
        } else {
            app.server_name_input.trim().to_string()
        };

        match GameServer::start(config, name) {
            Ok(srv) => {
                app.launcher_status.clear();
                return Some(AppMode::ServerHost(srv));
            }
            Err(e) => {
                app.launcher_status = format!("HOST FAILED: {} — is another instance hosting?", e);
            }
        }
    }

    if gui_button(
        p1_x + 30.0,
        p1_y + 350.0,
        420.0,
        50.0,
        "[>] DIRECT JOIN BY IP & PORT",
        Color::from_rgba(30, 45, 65, 255),
    ) {
        let port: u16 = app.target_port_input.parse().unwrap_or(5000);
        let name = if app.player_name_input.trim().is_empty() {
            "RustPlayer".to_string()
        } else {
            app.player_name_input.trim().to_string()
        };
        if let Ok(cli) = GameClient::connect(
            app.target_ip_input.trim().to_string(),
            port,
            name,
            app.server_name_input.clone(),
        ) {
            return Some(AppMode::ClientJoin(cli));
        }
    }

    let info_y = p1_y + 430.0;
    let info_color = Color::from_rgba(150, 160, 170, 255);
    draw_text(
        "• Server Mode: Hosts match & monitors connected players.",
        p1_x + 20.0,
        info_y,
        FONT_SIZE_SMALL,
        info_color,
    );
    draw_text(
        "• Client Mode: Connects to server to play.",
        p1_x + 20.0,
        info_y + 20.0,
        FONT_SIZE_SMALL,
        info_color,
    );
    draw_text(
        "• PRFC-v3 binary spec: 4-dir movement, 20Hz ticks.",
        p1_x + 20.0,
        info_y + 40.0,
        FONT_SIZE_SMALL,
        info_color,
    );

    let p2_x = 570.0;
    let p2_y = 130.0;
    let p2_w = 650.0;
    let p2_h = 530.0;
    gui_panel(
        p2_x,
        p2_y,
        p2_w,
        p2_h,
        "DISCOVERED LOCAL SERVERS (UDP BROADCAST)",
    );

    if gui_button(
        p2_x + p2_w - 180.0,
        p2_y + 5.0,
        170.0,
        24.0,
        "[R] REFRESH SCAN",
        Color::from_rgba(30, 45, 65, 255),
    ) {
        if let Some(ref sc) = app.scanner {
            sc.scan(5001);
        }
    }

    let servers = if let Some(ref sc) = app.scanner {
        sc.get_servers()
    } else {
        Vec::new()
    };

    if servers.is_empty() {
        draw_text(
            "Scanning local network for active servers on UDP port 5001...",
            p2_x + 30.0,
            p2_y + 80.0,
            FONT_SIZE_REGULAR,
            Color::from_rgba(160, 180, 200, 255),
        );
        draw_text(
            "No servers found yet. Host a new server or use Direct Join.",
            p2_x + 30.0,
            p2_y + 110.0,
            FONT_SIZE_SMALL,
            Color::from_rgba(120, 140, 160, 255),
        );
    } else {
        let mut sy = p2_y + 50.0;
        for srv in servers {
            gui_panel(p2_x + 20.0, sy, p2_w - 40.0, 60.0, "");

            draw_text(
                &format!("{} ({}:{})", srv.server_name, srv.ip, srv.tcp_port),
                p2_x + 35.0,
                sy + 25.0,
                FONT_SIZE_MEDIUM,
                WHITE,
            );

            let state_str = match srv.state {
                GameState::Waiting => "WAITING (Open)",
                GameState::Starting => "STARTING...",
                GameState::Running => "IN GAME",
                _ => "OTHER",
            };

            draw_text(
                &format!(
                    "Players: {}/{} | Status: {} | Game ID: {}",
                    srv.player_count, srv.max_players, state_str, srv.game_id,
                ),
                p2_x + 35.0,
                sy + 45.0,
                FONT_SIZE_SMALL,
                COLOR_UI_ACCENT_CYAN,
            );

            if srv.state == GameState::Waiting {
                if gui_button(
                    p2_x + p2_w - 150.0,
                    sy + 12.0,
                    110.0,
                    36.0,
                    "JOIN >>",
                    Color::from_rgba(30, 45, 65, 255),
                ) {
                    let name = if app.player_name_input.trim().is_empty() {
                        "RustPlayer".to_string()
                    } else {
                        app.player_name_input.trim().to_string()
                    };
                    if let Ok(cli) =
                        GameClient::connect(srv.ip, srv.tcp_port, name, srv.server_name)
                    {
                        return Some(AppMode::ClientJoin(cli));
                    }
                }
            }
            sy += 70.0;
        }
    }
    None
}
