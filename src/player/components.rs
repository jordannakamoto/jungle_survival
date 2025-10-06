use bevy::prelude::*;

#[derive(Resource)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub vx: f32,
    pub vy: f32,
    pub width: i32,
    pub height: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 200,  // Spawn away from sand pile at x=400
            y: 500,
            vx: 0.0,
            vy: 0.0,
            width: 8,
            height: 16,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.insert_resource(Player::default());
}
