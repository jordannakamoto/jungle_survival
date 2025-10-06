use bevy::prelude::*;
use crate::player::components::Player;
use crate::input::GameInput;
use crate::world::PixelWorld;

const GRAVITY: f32 = 600.0;
const PLAYER_SPEED: f32 = 150.0;
const JUMP_FORCE: f32 = 300.0;

pub fn player_movement(
    mut player: ResMut<Player>,
    game_input: Res<GameInput>,
    world: Res<PixelWorld>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    // Apply gravity
    player.vy += GRAVITY * dt;

    // Horizontal input
    player.vx = game_input.movement.x * PLAYER_SPEED;

    // Apply velocities
    let new_x = player.x as f32 + player.vx * dt;
    let new_y = player.y as f32 + player.vy * dt;

    // Horizontal collision detection
    let mut final_x = new_x as i32;
    let can_move_x = check_horizontal_collision(&world, &player, final_x);
    if !can_move_x {
        final_x = player.x;
        player.vx = 0.0;
    }

    // Vertical collision detection
    let mut final_y = new_y as i32;
    let ground_collision = check_ground_collision(&world, &player, final_x, final_y);
    let ceiling_collision = check_ceiling_collision(&world, &player, final_x, final_y);

    if ground_collision {
        // On ground - stop falling
        final_y = player.y;
        player.vy = 0.0;

        // Allow jumping
        if game_input.movement.y > 0.0 {
            player.vy = -JUMP_FORCE;
        }
    } else if ceiling_collision {
        // Hit ceiling - stop upward movement
        final_y = player.y;
        player.vy = player.vy.max(0.0);
    }

    // Apply final position
    player.x = final_x;
    player.y = final_y;
}

fn check_horizontal_collision(world: &PixelWorld, player: &Player, new_x: i32) -> bool {
    // Check left and right sides
    for dy in 0..player.height {
        let check_y = player.y + dy - player.height / 2;

        // Left side
        if world.get(new_x - player.width / 2 - 1, check_y).is_solid() {
            return false;
        }

        // Right side
        if world.get(new_x + player.width / 2, check_y).is_solid() {
            return false;
        }
    }
    true
}

fn check_ground_collision(world: &PixelWorld, player: &Player, x: i32, y: i32) -> bool {
    // Check bottom of player
    for dx in 0..player.width {
        let check_x = x + dx - player.width / 2;
        let check_y = y + player.height / 2 + 1;

        if world.get(check_x, check_y).is_solid() {
            return true;
        }
    }
    false
}

fn check_ceiling_collision(world: &PixelWorld, player: &Player, x: i32, y: i32) -> bool {
    // Check top of player
    for dx in 0..player.width {
        let check_x = x + dx - player.width / 2;
        let check_y = y - player.height / 2 - 1;

        if world.get(check_x, check_y).is_solid() {
            return true;
        }
    }
    false
}
