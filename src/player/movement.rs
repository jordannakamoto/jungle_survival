use bevy::prelude::*;
use crate::player::components::Player;
use crate::input::GameInput;
use crate::world::{PixelWorld, Material};

const GRAVITY: f32 = 600.0;
const PLAYER_SPEED: f32 = 150.0;
const JUMP_FORCE: f32 = 300.0;
const MAX_SLOPE_HEIGHT: i32 = 6; // Maximum pixels the player can auto-climb per step

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

    // Debug movement
    if game_input.movement.x != 0.0 {
        println!("Movement - input.x: {}, vx: {}, dt: {}, player.x: {}",
            game_input.movement.x, player.vx, dt, player.x);
    }

    // Apply velocities
    let new_x = player.x + player.vx * dt;
    let new_y = player.y + player.vy * dt;

    let mut final_x = new_x;
    let mut final_y = new_y;

    // Try horizontal movement first
    let can_move_x = check_horizontal_collision(&world, &player, new_x as i32);
    if !can_move_x {
        // Blocked horizontally - try to climb
        if let Some(climb_height) = check_slope(&world, &player, new_x as i32) {
            // Can climb! Move up and forward
            final_x = new_x;
            final_y = player.y - climb_height as f32; // Move up (negative Y is up)
        } else {
            // Can't climb, stop horizontal movement
            final_x = player.x;
            player.vx = 0.0;
        }
    }

    // Check ceiling collision at new position
    let ceiling_collision = check_ceiling_collision(&world, &player, final_x as i32, final_y as i32);
    if ceiling_collision {
        // Hit ceiling - stop upward movement
        final_y = player.y;
        player.vy = player.vy.max(0.0);
    }

    // Ground collision and slope adjustment
    let ground_collision = check_ground_collision(&world, &player, final_x as i32, final_y as i32);
    if ground_collision {
        // We're colliding with ground - find the exact ground level
        // Only search upward if we've actually moved horizontally (walking onto slope)
        let moved_horizontally = (final_x - player.x).abs() > 0.5;

        if moved_horizontally {
            // Walking onto a slope - search upward to find surface
            let mut ground_y = final_y as i32;
            for search_offset in 0..MAX_SLOPE_HEIGHT {
                let test_y = final_y as i32 - search_offset;
                if !check_ground_collision(&world, &player, final_x as i32, test_y) {
                    // Found the first position where we're NOT in ground
                    ground_y = test_y;
                    break;
                }
            }
            final_y = ground_y as f32;
        } else {
            // Not moving horizontally - just snap to current position
            // This prevents jittering from gravity when standing still
            final_y = player.y;
        }

        player.vy = 0.0;

        // Allow jumping
        if game_input.movement.y > 0.0 {
            player.vy = -JUMP_FORCE;
        }
    }

    // Apply final position
    player.x = final_x;
    player.y = final_y;
}

fn check_horizontal_collision(world: &PixelWorld, player: &Player, new_x: i32) -> bool {
    // Only collide with ground (dirt) - can pass through trees and sand
    for dy in 0..player.height {
        let check_y = player.y as i32 + dy - player.height / 2;

        // Left side
        if world.get(new_x - player.width / 2 - 1, check_y) == Material::Dirt {
            return false;
        }

        // Right side (symmetric with left)
        if world.get(new_x + player.width / 2 + 1, check_y) == Material::Dirt {
            return false;
        }
    }
    true
}

fn check_ground_collision(world: &PixelWorld, player: &Player, x: i32, y: i32) -> bool {
    // Only stand on dirt - can pass through trees and sand
    for dx in 0..player.width {
        let check_x = x + dx - player.width / 2;
        let check_y = y + player.height / 2 + 1;

        if world.get(check_x, check_y) == Material::Dirt {
            return true;
        }
    }
    false
}

fn check_ceiling_collision(world: &PixelWorld, player: &Player, x: i32, y: i32) -> bool {
    // Only collide with dirt ceiling - can pass through trees and sand
    for dx in 0..player.width {
        let check_x = x + dx - player.width / 2;
        let check_y = y - player.height / 2 - 1;

        if world.get(check_x, check_y) == Material::Dirt {
            return true;
        }
    }
    false
}

/// Check if the player can climb a slope at the new x position
/// Returns Some(height) if climbable, None if too steep or blocked
fn check_slope(world: &PixelWorld, player: &Player, new_x: i32) -> Option<i32> {
    // Check each height from 1 to MAX_SLOPE_HEIGHT
    for climb_height in 1..=MAX_SLOPE_HEIGHT {
        let test_y = player.y as i32 - climb_height; // Try moving up

        // Check if at this elevated position, there's no wall blocking horizontally
        let mut has_horizontal_space = true;
        for dy in 0..player.height {
            let check_y = test_y + dy - player.height / 2;

            // Check left and right sides at the new position
            if world.get(new_x - player.width / 2 - 1, check_y) == Material::Dirt ||
               world.get(new_x + player.width / 2 + 1, check_y) == Material::Dirt {
                has_horizontal_space = false;
                break;
            }
        }

        // Check if there's no ceiling at this height
        let mut has_ceiling = false;
        for dx in 0..player.width {
            let check_x = new_x + dx - player.width / 2;
            let check_y = test_y - player.height / 2 - 1;

            if world.get(check_x, check_y) == Material::Dirt {
                has_ceiling = true;
                break;
            }
        }

        // If we have space horizontally and no ceiling, this height works!
        if has_horizontal_space && !has_ceiling {
            return Some(climb_height);
        }
    }

    // Slope is too steep or blocked
    None
}
