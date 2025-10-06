use bevy::prelude::*;
use crate::input::GameInput;
use crate::pixel_world::{PixelWorld, Material};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Player { x: 400, y: 500, vx: 0.0, vy: 0.0 })
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (player_physics, player_input, interact_with_world));
    }
}

#[derive(Resource)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub vx: f32,
    pub vy: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn player_physics(
    mut player: ResMut<Player>,
    world: Res<PixelWorld>,
    time: Res<Time>,
) {
    let gravity = 400.0;
    player.vy += gravity * time.delta_secs();

    // Apply velocity
    player.x = (player.x as f32 + player.vx * time.delta_secs()) as i32;
    player.y = (player.y as f32 + player.vy * time.delta_secs()) as i32;

    // Simple collision with solid pixels
    let player_width = 8;
    let player_height = 16;

    // Check if standing on solid ground
    let mut on_ground = false;
    for dx in 0..player_width {
        if world.get(player.x + dx - player_width / 2, player.y + player_height / 2 + 1).is_solid() {
            on_ground = true;
            player.vy = 0.0;
            break;
        }
    }

    // Check head collision
    for dx in 0..player_width {
        if world.get(player.x + dx - player_width / 2, player.y - player_height / 2 - 1).is_solid() {
            player.vy = player.vy.max(0.0);
            break;
        }
    }

    // Check side collisions
    if world.get(player.x - player_width / 2 - 1, player.y).is_solid() {
        player.vx = player.vx.max(0.0);
    }
    if world.get(player.x + player_width / 2 + 1, player.y).is_solid() {
        player.vx = player.vx.min(0.0);
    }
}

fn player_input(
    mut player: ResMut<Player>,
    game_input: Res<GameInput>,
    world: Res<PixelWorld>,
) {
    let speed = 100.0;
    let jump_force = 250.0;

    // Horizontal movement
    player.vx = game_input.movement.x * speed;

    // Jump - check if on ground first
    let player_width = 8;
    let player_height = 16;
    let mut on_ground = false;
    for dx in 0..player_width {
        if world.get(player.x + dx - player_width / 2, player.y + player_height / 2 + 1).is_solid() {
            on_ground = true;
            break;
        }
    }

    if game_input.movement.y > 0.0 && on_ground {
        player.vy = -jump_force;
    }
}

fn interact_with_world(
    player: Res<Player>,
    mut gizmos: Gizmos,
) {
    // Draw player as rectangle
    let player_width = 8.0;
    let player_height = 16.0;
    let px = player.x as f32 - 400.0;
    let py = 300.0 - player.y as f32;

    gizmos.rect_2d(
        Isometry2d::new(Vec2::new(px, py), Rot2::IDENTITY),
        Vec2::new(player_width, player_height),
        Color::srgb(0.9, 0.7, 0.5),
    );
}
