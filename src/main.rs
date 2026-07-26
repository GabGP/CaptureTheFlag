mod client;
mod config;
mod gui;
mod protocol;
mod server;
mod states;
mod types;

use crate::{
    client::{
        client::GameClient,
        client_ui::{client_running, client_start},
    },
    gui::logs::draw_logs,
    server::{
        server::GameServer,
        server_ui::{server_running, server_start},
    },
    states::app_state::AppState,
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

// ============================================================================
// MAIN APPLICATION
// ============================================================================

#[macroquad::main("Capture The Flag")]
async fn main() {
    let mut app_state = AppState::MainMenu;

    // UI Input fields
    let mut ip_input = "127.0.0.1".to_string();
    let mut port_input = "5000".to_string();
    let mut discovery_port_input = "5001".to_string();
    let mut server_name_input = "Game Server".to_string();
    let mut name_input = "MacroquadTest".to_string();

    // Server and Client state wrappers
    let mut game_server: Option<GameServer> = None;
    let mut game_client: Option<GameClient> = None;

    // Shared Logs
    let mut logs: Vec<String> = Vec::new();

    loop {
        clear_background(BLACK);

        match app_state {
            AppState::MainMenu => {
                let center_x = screen_width() / 2.0;
                let center_y = screen_height() / 2.0;

                root_ui().window(
                    hash!(),
                    vec2(center_x - 100., center_y - 75.),
                    vec2(200., 150.),
                    |ui| {
                        if ui.button(vec2(50., 30.), "CREATE GAME") {
                            app_state = AppState::CreateGame;
                        }
                        if ui.button(vec2(55., 80.), "JOIN GAME") {
                            app_state = AppState::JoinGame;
                        }
                    },
                );
            }

            AppState::CreateGame => {
                server_start(
                    &mut port_input,
                    &mut discovery_port_input,
                    &mut server_name_input,
                    &mut game_server,
                    &mut logs,
                    &mut app_state,
                );
            }

            AppState::JoinGame => {
                client_start(
                    &mut ip_input,
                    &mut port_input,
                    &mut name_input,
                    &mut game_client,
                    &mut logs,
                    &mut app_state,
                );
            }

            AppState::ClientRunning => {
                client_running(&mut game_client, &mut app_state, &mut logs);
            }

            AppState::ServerRunning => {
                server_running(&mut game_server, &mut app_state, &mut logs);
            }
        }

        draw_logs(&logs);
        next_frame().await
    }
}
