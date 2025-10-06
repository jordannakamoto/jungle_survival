pub mod components;
pub mod movement;
pub mod rendering;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, components::spawn_player)
            .add_systems(Update, (
                movement::player_movement,
                rendering::render_player,
            ));
    }
}
