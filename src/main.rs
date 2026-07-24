mod client;
mod config;
mod game;
mod gui;
mod server;
mod states;

use crate::{
    client::client::{client_running, client_start},
    game::board::Board,
    gui::logs::draw_logs,
    server::server::{server_running, server_start},
    states::{app_state::AppState, game_state::GameState},
};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

use std::net::{TcpListener, TcpStream};

// ============================================================================
// MAIN APPLICATION
// ============================================================================

#[macroquad::main("Capture The Flag")]
async fn main() {
    let mut app_state = AppState::MainMenu;
    let mut game_state = GameState::Waiting;

    // UI Input fields
    let mut ip_input = "127.0.0.1".to_string();
    let mut port_input = "5000".to_string();
    let mut name_input = "MacroquadTest".to_string();

    // Client State Variables
    let mut tcp_stream: Option<TcpStream> = None;
    let mut buffer = String::new();
    let mut current_player_id = String::new();
    let mut current_game_id = String::new();

    // Server State Variables
    let mut tcp_listener: Option<TcpListener> = None;
    let mut active_clients: Vec<TcpStream> = Vec::new();
    let mut last_tick_time = 0.0;
    let mut tick_counter = 0;

    // Shared Variables
    let mut board: Option<Board> = None;
    let mut logs: Vec<String> = Vec::new();
    let mut game_started = false;

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
                    &mut ip_input,
                    &mut port_input,
                    &mut tcp_listener,
                    &mut logs,
                    &mut app_state,
                );
            }

            AppState::JoinGame => {
                client_start(
                    &mut ip_input,
                    &mut port_input,
                    &mut name_input,
                    &mut tcp_stream,
                    &mut logs,
                    &mut app_state,
                );
            }

            AppState::ClientRunning => {
                client_running(
                    &mut tcp_stream,
                    &mut logs,
                    &mut app_state,
                    &mut current_player_id,
                    &mut current_game_id,
                    &mut buffer,
                    &mut game_started,
                );
            }

            AppState::ServerRunning => {
                server_running(
                    &mut tcp_listener,
                    &mut active_clients,
                    &mut logs,
                    &mut app_state,
                    &mut game_state,
                    &mut board,
                    &mut last_tick_time,
                    &mut tick_counter,
                );
            }
        }

        draw_logs(&logs);
        next_frame().await
    }
}
