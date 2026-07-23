mod app_state;
mod board;
mod client;
mod game_state;
mod logs;
mod server;

use crate::{
    client::client::{client_running, client_start},
    server::server::{server_running, server_start},
};

use app_state::AppState;
use board::Board;
use game_state::GameState;
use logs::draw_logs;

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
    let mut direction_sent = false;

    // Server State Variables
    let mut tcp_listener: Option<TcpListener> = None;
    let mut active_clients: Vec<TcpStream> = Vec::new();

    // Shared Variables
    let mut board: Option<Board> = None;
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
                draw_logs(&logs);
            }

            AppState::CreateGame => {
                server_start(
                    &mut ip_input,
                    &mut port_input,
                    &mut tcp_listener,
                    &mut logs,
                    &mut app_state,
                );
                draw_logs(&logs);
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
                draw_logs(&logs);
            }

            AppState::ClientRunning => {
                client_running(
                    &mut tcp_stream,
                    &mut logs,
                    &mut app_state,
                    &mut current_player_id,
                    &mut current_game_id,
                    &mut buffer,
                    &mut direction_sent,
                );
                draw_logs(&logs);
            }

            AppState::ServerRunning => {
                server_running(
                    &mut tcp_listener,
                    &mut active_clients,
                    &mut logs,
                    &mut app_state,
                    &mut game_state,
                    &mut board,
                );
                draw_logs(&logs);
            }
        }

        next_frame().await
    }
}
