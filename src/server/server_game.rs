use crate::{
    protocol::{protocol::Message, types::*},
    server::server_net::broadcast_msg,
};
use std::collections::HashMap;
use std::net::TcpStream;
use std::time::{Duration, Instant};

// ============================================================================
// SERVER GAME
// ============================================================================

/// Function to handle starting countdown transitions and player initializations
pub fn update_countdown_state(
    state: &mut GameState,
    countdown_sec: &mut u8,
    last_countdown_instant: &mut Instant,
    tick: &mut u32,
    players: &mut HashMap<u16, PlayerState>,
    player_streams: &mut HashMap<u16, TcpStream>,
    flag_status: &mut FlagStatus,
    flag_carrier_id: &mut u16,
    flag_x: &mut f32,
    flag_y: &mut f32,
    logs: &mut Vec<String>,
    config: &GameConfig,
) {
    if *state == GameState::Starting {
        if last_countdown_instant.elapsed() >= Duration::from_secs(1) {
            *last_countdown_instant = Instant::now();
            if *countdown_sec > 1 {
                *countdown_sec -= 1;
                let msg = Message::GameCountdown {
                    seconds_remaining: *countdown_sec,
                };
                broadcast_msg(player_streams, &msg);
            } else {
                *state = GameState::Running;
                *tick = 0;

                let spawn_dist = config.circle_radius + config.spawn_margin;
                for p in players.values_mut() {
                    let angle: f32 = ::rand::random_range(0.0..std::f32::consts::TAU);
                    p.x = (spawn_dist * angle.cos())
                        .clamp(-config.map_size / 2.0, config.map_size / 2.0);
                    p.y = (spawn_dist * angle.sin())
                        .clamp(-config.map_size / 2.0, config.map_size / 2.0);
                    p.direction = Direction::None;
                    p.has_flag = false;
                }

                *flag_status = FlagStatus::Available;
                *flag_carrier_id = 0;
                *flag_x = 0.0;
                *flag_y = 0.0;

                logs.push("Game STARTED!".to_string());
                let game_started_msg = Message::GameStarted {
                    config: config.clone(),
                    flag_status: *flag_status,
                    flag_carrier_id: 0,
                    flag_x: 0.0,
                    flag_y: 0.0,
                    players: players.values().cloned().collect(),
                };
                broadcast_msg(player_streams, &game_started_msg);
            }
        }
    }
}

/// Function to handle running game tick physics, interactions, and win conditions
pub fn update_running_game_state(
    state: &mut GameState,
    tick: &mut u32,
    players: &mut HashMap<u16, PlayerState>,
    player_streams: &mut HashMap<u16, TcpStream>,
    flag_status: &mut FlagStatus,
    flag_carrier_id: &mut u16,
    flag_x: &mut f32,
    flag_y: &mut f32,
    logs: &mut Vec<String>,
    config: &GameConfig,
    pending_inputs: &mut HashMap<u16, Direction>,
    pending_interacts: &mut HashMap<u16, Instant>,
    last_tick_time: &mut Instant,
    tick_dur: Duration,
) {
    if *state == GameState::Running && last_tick_time.elapsed() >= tick_dur {
        *last_tick_time = Instant::now();
        *tick += 1;

        for (pid, dir) in pending_inputs.drain() {
            if let Some(p) = players.get_mut(&pid) {
                p.direction = dir;
            }
        }

        let paso = config.player_speed * (config.tick_interval_ms as f32) / 1000.0;
        let half_map = config.map_size / 2.0;

        for p in players.values_mut() {
            match p.direction {
                Direction::Up => p.y -= paso,
                Direction::Down => p.y += paso,
                Direction::Left => p.x -= paso,
                Direction::Right => p.x += paso,
                Direction::None => {}
            }
            p.x = p.x.clamp(-half_map, half_map);
            p.y = p.y.clamp(-half_map, half_map);

            if p.has_flag {
                *flag_x = p.x;
                *flag_y = p.y;
            }
        }

        let mut sorted_interacts: Vec<u16> = pending_interacts.keys().cloned().collect();
        sorted_interacts.sort_unstable();
        pending_interacts.clear();

        let mut steal_occurred = false;
        for pid in sorted_interacts {
            if let Some(p) = players.get(&pid) {
                let px = p.x;
                let py = p.y;

                if *flag_status == FlagStatus::Available || *flag_status == FlagStatus::Dropped {
                    let dist = ((px - *flag_x).powi(2) + (py - *flag_y).powi(2)).sqrt();
                    if dist <= config.interaction_radius {
                        *flag_status = FlagStatus::Carried;
                        *flag_carrier_id = pid;
                        if let Some(mut_p) = players.get_mut(&pid) {
                            mut_p.has_flag = true;
                        }
                        *flag_x = px;
                        *flag_y = py;
                        logs.push(format!("Player ID {} picked up the flag!", pid));
                        broadcast_msg(
                            player_streams,
                            &Message::FlagPickedUp {
                                tick: *tick,
                                player_id: pid,
                            },
                        );
                    }
                } else if *flag_status == FlagStatus::Carried
                    && *flag_carrier_id != pid
                    && !steal_occurred
                {
                    if let Some(carrier) = players.get(&*flag_carrier_id) {
                        let dist = ((px - carrier.x).powi(2) + (py - carrier.y).powi(2)).sqrt();
                        if dist <= config.interaction_radius {
                            let prev_id = *flag_carrier_id;
                            steal_occurred = true;
                            if let Some(old_c) = players.get_mut(&prev_id) {
                                old_c.has_flag = false;
                            }
                            if let Some(new_c) = players.get_mut(&pid) {
                                new_c.has_flag = true;
                            }
                            *flag_carrier_id = pid;
                            *flag_x = px;
                            *flag_y = py;
                            logs.push(format!("Player ID {} stole flag from ID {}!", pid, prev_id));
                            broadcast_msg(
                                player_streams,
                                &Message::FlagStolen {
                                    tick: *tick,
                                    previous_carrier_id: prev_id,
                                    new_carrier_id: pid,
                                },
                            );
                        }
                    }
                }
            }
        }

        let mut winner: Option<(u16, String)> = None;
        if *flag_status == FlagStatus::Carried {
            if let Some(carrier) = players.get(&*flag_carrier_id) {
                let dist_origin = (carrier.x.powi(2) + carrier.y.powi(2)).sqrt();
                if dist_origin - config.player_radius > config.circle_radius {
                    winner = Some((carrier.player_id, carrier.name.clone()));
                }
            }
        }

        if let Some((win_id, win_name)) = winner {
            *state = GameState::Finished;
            *flag_status = FlagStatus::Outside;
            logs.push(format!("GAME OVER! Winner: {} (ID {})", win_name, win_id));

            let game_state_msg = Message::GameStateMsg {
                tick: *tick,
                flag_status: *flag_status,
                flag_carrier_id: *flag_carrier_id,
                flag_x: *flag_x,
                flag_y: *flag_y,
                players: players.values().cloned().collect(),
            };
            broadcast_msg(player_streams, &game_state_msg);

            let game_over_msg = Message::GameOver {
                winner_id: win_id,
                winner_name: win_name,
                reason: GameOverReason::ExitedCircleWithFlag,
            };
            broadcast_msg(player_streams, &game_over_msg);
        } else {
            let game_state_msg = Message::GameStateMsg {
                tick: *tick,
                flag_status: *flag_status,
                flag_carrier_id: *flag_carrier_id,
                flag_x: *flag_x,
                flag_y: *flag_y,
                players: players.values().cloned().collect(),
            };
            broadcast_msg(player_streams, &game_state_msg);
        }
    }
}
