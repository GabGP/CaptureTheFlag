use super::{
    camera::Camera2DWorld,
    render_players::render_players,
    render_world::{render_background_and_grid, render_central_circle, render_flag},
};
use crate::protocol::types::*;

// ============================================================================
// RENDER
// ============================================================================

/// Function to render the game world
pub fn render_game_world(
    cam: &Camera2DWorld,
    config: &GameConfig,
    players: &[PlayerState],
    flag_status: FlagStatus,
    _flag_carrier_id: u16,
    flag_x: f32,
    flag_y: f32,
    local_player_id: Option<u16>,
    time: f32,
) {
    render_background_and_grid(cam, config);
    render_central_circle(cam, config, time);
    render_flag(cam, config, flag_status, flag_x, flag_y, time);
    render_players(cam, config, players, local_player_id);
}
