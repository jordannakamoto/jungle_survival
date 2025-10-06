use bevy::prelude::*;
use crate::player::components::Player;

pub fn render_player(
    player: Res<Player>,
    mut gizmos: Gizmos,
) {
    let player_width = player.width as f32;
    let player_height = player.height as f32;

    // Convert pixel coordinates to world coordinates
    let px = player.x - 400.0;
    let py = 300.0 - player.y;

    gizmos.rect_2d(
        Isometry2d::new(Vec2::new(px, py), Rot2::IDENTITY),
        Vec2::new(player_width, player_height),
        Color::srgb(0.9, 0.7, 0.5),
    );
}
