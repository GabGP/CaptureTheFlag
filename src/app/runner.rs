use crate::{
    app::mode_client, client::client::GameClient, gui::camera::Camera2DWorld, protocol::network::udp_utils::UdpScanner, server::server::GameServer,
};
use macroquad::prelude::*;

use super::{mode_launcher, mode_server};

// ============================================================================
// APP LAUNCHER STATE
// ============================================================================

pub enum AppMode {
    Launcher,
    ServerHost(GameServer),
    ClientJoin(GameClient),
}

pub struct AppRunner {
    pub mode: AppMode,
    pub player_name_input: String,
    pub server_name_input: String,
    pub target_ip_input: String,
    pub target_port_input: String,
    pub focus_field: i32,
    pub launcher_status: String,
    pub scanner: Option<UdpScanner>,
    pub last_scan_time: f64,
    pub camera: Camera2DWorld,
}

impl AppRunner {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Launcher,
            player_name_input: "RustWarrior".to_string(),
            server_name_input: "Rust Host - Group 1".to_string(),
            target_ip_input: "127.0.0.1".to_string(),
            target_port_input: "5000".to_string(),
            focus_field: 0,
            launcher_status: String::new(),
            scanner: UdpScanner::new(5001).ok(),
            last_scan_time: get_time(),
            camera: Camera2DWorld::new(),
        }
    }

    pub async fn update(&mut self) {
        let time = get_time() as f32;
        let mut next_mode = None;

        if matches!(self.mode, AppMode::Launcher) {
            next_mode = mode_launcher::update(self, time);
        } else if let AppMode::ServerHost(ref mut server) = self.mode {
            next_mode = mode_server::update(server, &mut self.camera, time);
        } else if let AppMode::ClientJoin(ref mut client) = self.mode {
            next_mode = mode_client::update(client, &mut self.camera, time);
        }

        if let Some(new_mode) = next_mode {
            self.mode = new_mode;
        }
    }
}