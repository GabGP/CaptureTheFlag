use crate::{
    debugger::{LogDirection, log_client_message},
    protocol::{protocol, types::*},
};
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================================
// UDP DISCOVERY SCANNER
// ============================================================================

pub struct UdpScanner {
    socket: UdpSocket,
    discovered: Arc<Mutex<HashMap<String, DiscoveredServer>>>,
}

impl UdpScanner {
    /// Function to create a new UDP scanner and spawn a background listener thread
    pub fn new(_discovery_port: u16) -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;
        socket.set_nonblocking(true)?;

        let discovered = Arc::new(Mutex::new(HashMap::new()));
        let disc_writer = discovered.clone();
        let sock_clone = socket.try_clone()?;

        thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match sock_clone.recv_from(&mut buf) {
                    Ok((len, src_addr)) => {
                        if let Ok(msg) = protocol::Message::deserialize(&buf[..len]) {
                            if let protocol::Message::DiscoverResponse {
                                game_id,
                                server_name,
                                tcp_port,
                                state,
                                player_count,
                                maximum_players,
                            } = msg
                            {
                                let ip = src_addr.ip().to_string();
                                log_client_message(
                                    &ip,
                                    LogDirection::Received,
                                    "DiscoverResponse",
                                    &format!(
                                        "server_name={}, tcp_port={}, state={:?}, player_count={}, maximum_players={}",
                                        server_name, tcp_port, state, player_count, maximum_players
                                    ),
                                );
                                let key = format!("{}:{}", ip, tcp_port);
                                let server_info = DiscoveredServer {
                                    ip,
                                    game_id,
                                    server_name,
                                    tcp_port,
                                    state,
                                    player_count,
                                    max_players: maximum_players,
                                    last_seen: Instant::now(),
                                };
                                disc_writer.lock().unwrap().insert(key, server_info);
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(50));
                    }
                    // Ignore ICMP unreachable errors on Windows
                    Err(ref e) if e.kind() == io::ErrorKind::ConnectionReset => {
                        thread::sleep(Duration::from_millis(50));
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self { socket, discovered })
    }

    /// Function to broadcast a network ping asking local servers to identify themselves
    pub fn scan(&self, discovery_port: u16) {
        let req = protocol::Message::DiscoverRequest;
        let bytes = req.serialize();
        if let Ok(target) = format!("255.255.255.255:{}", discovery_port).parse::<SocketAddr>() {
            let _ = self.socket.send_to(&bytes, target);
            log_client_message(
                &target.to_string(),
                LogDirection::Sent,
                "DiscoverRequest",
                "broadcast discovery request",
            );
        }
        if let Ok(local_target) = format!("127.0.0.1:{}", discovery_port).parse::<SocketAddr>() {
            let _ = self.socket.send_to(&bytes, local_target);
            log_client_message(
                &local_target.to_string(),
                LogDirection::Sent,
                "DiscoverRequest",
                "local discovery request",
            );
        }
    }

    /// Function to retrieve and return a cleaned-up list of active servers found nearby
    pub fn get_servers(&self) -> Vec<DiscoveredServer> {
        let mut map = self.discovered.lock().unwrap();
        // filter out stale servers (> 5 seconds old)
        map.retain(|_, s| s.last_seen.elapsed() < Duration::from_secs(5));
        let mut list: Vec<DiscoveredServer> = map.values().cloned().collect();
        list.sort_by(|a, b| a.server_name.cmp(&b.server_name));
        list
    }
}
