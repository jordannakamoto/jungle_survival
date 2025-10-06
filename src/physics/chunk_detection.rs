use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::world::{PixelWorld, Material};
use super::components::WoodChunk;
use std::collections::{HashSet, VecDeque};

/// Check for wood that's disconnected from ground and convert to rigid bodies
pub fn detect_floating_chunks(
    mut commands: Commands,
    mut world: ResMut<PixelWorld>,
    time: Res<Time>,
) {
    // Only check every 0.5 seconds to avoid performance issues
    if time.elapsed_secs() % 0.5 > 0.1 {
        return;
    }

    let width = world.width as i32;
    let height = world.height as i32;

    // Find all wood pixels that are grounded (connected to solid ground like dirt)
    let mut grounded = HashSet::new();
    let mut to_check = VecDeque::new();

    // Find wood that's touching ground (dirt, sand, or other non-wood solid)
    for y in 0..height {
        for x in 0..width {
            if world.get(x, y) == Material::Wood {
                // Check if this wood is touching solid ground (non-air, non-wood)
                let mut touches_ground = false;
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    let material = world.get(nx, ny);

                    // Wood is grounded if it touches dirt or sand
                    if material == Material::Dirt || material == Material::Sand {
                        touches_ground = true;
                        break;
                    }
                }

                if touches_ground {
                    grounded.insert((x, y));
                    to_check.push_back((x, y));
                }
            }
        }
    }

    // Flood fill to find all grounded wood
    while let Some((x, y)) = to_check.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                if world.get(nx, ny) == Material::Wood && !grounded.contains(&(nx, ny)) {
                    grounded.insert((nx, ny));
                    to_check.push_back((nx, ny));
                }
            }
        }
    }

    // Find floating wood chunks
    let mut visited = HashSet::new();
    let mut chunks_to_spawn = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if world.get(x, y) == Material::Wood
                && !grounded.contains(&(x, y))
                && !visited.contains(&(x, y))
            {
                // Found a floating chunk - flood fill to get all connected pixels
                let chunk_pixels = flood_fill_chunk(&world, x, y, &mut visited);

                if chunk_pixels.len() >= 5 {
                    // Only spawn chunks with at least 5 pixels
                    chunks_to_spawn.push(chunk_pixels);
                }
            }
        }
    }

    // Spawn rigid bodies for each chunk
    for chunk_pixels in chunks_to_spawn {
        spawn_wood_chunk(&mut commands, &mut world, chunk_pixels);
    }
}

fn flood_fill_chunk(
    world: &PixelWorld,
    start_x: i32,
    start_y: i32,
    visited: &mut HashSet<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut chunk = Vec::new();
    let mut to_check = VecDeque::new();

    to_check.push_back((start_x, start_y));
    visited.insert((start_x, start_y));

    while let Some((x, y)) = to_check.pop_front() {
        chunk.push((x, y));

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;

            if world.get(nx, ny) == Material::Wood && !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                to_check.push_back((nx, ny));
            }
        }
    }

    chunk
}

fn spawn_wood_chunk(
    commands: &mut Commands,
    world: &mut PixelWorld,
    pixels: Vec<(i32, i32)>,
) {
    if pixels.is_empty() {
        return;
    }

    // Calculate center of mass
    let sum_x: i32 = pixels.iter().map(|(x, _)| x).sum();
    let sum_y: i32 = pixels.iter().map(|(_, y)| y).sum();
    let count = pixels.len() as i32;
    let center_x = sum_x / count;
    let center_y = sum_y / count;

    // Convert pixel coordinates to world coordinates
    let world_x = center_x as f32 - 400.0;
    let world_y = 300.0 - center_y as f32;

    // Create convex hull approximation from pixels for collider
    // For now, use simplified cuboid based on bounding box
    let min_x = pixels.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = pixels.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = pixels.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = pixels.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x + 1) as f32;
    let height = (max_y - min_y + 1) as f32;

    // Calculate if this is a tall vertical structure (like a tree trunk)
    let is_tree_like = height > width * 1.5;

    // Determine fall direction based on center of mass offset
    let bottom_center_x: i32 = pixels.iter()
        .filter(|(_, y)| *y > center_y) // Bottom half
        .map(|(x, _)| x)
        .sum::<i32>() / pixels.iter().filter(|(_, y)| *y > center_y).count().max(1) as i32;

    let top_center_x: i32 = pixels.iter()
        .filter(|(_, y)| *y < center_y) // Top half
        .map(|(x, _)| x)
        .sum::<i32>() / pixels.iter().filter(|(_, y)| *y < center_y).count().max(1) as i32;

    // Calculate initial angular velocity for realistic tree falling
    let mut angular_velocity = 0.0;
    let mut initial_torque = Vec2::ZERO;

    if is_tree_like {
        // Tree-like structures should tip over
        let offset = top_center_x - bottom_center_x;

        // Determine which way to fall
        let fall_direction = if offset.abs() > 2 {
            offset.signum() as f32
        } else {
            // If balanced, pick a random direction
            if center_x % 2 == 0 { 1.0 } else { -1.0 }
        };

        // Initial angular velocity to start the tipping motion
        angular_velocity = fall_direction * 0.5;

        // Add a horizontal impulse to help the tree fall over
        initial_torque = Vec2::new(fall_direction * 50.0, 0.0);
    }

    // Remove pixels from pixel world
    for (x, y) in &pixels {
        world.set(*x, *y, Material::Air);
    }

    // Spawn rigid body with realistic tree falling physics
    commands.spawn((
        Transform::from_xyz(world_x, world_y, 1.0),
        RigidBody::Dynamic,
        Collider::cuboid(width / 2.0, height / 2.0),
        Velocity {
            linvel: initial_torque,
            angvel: angular_velocity,
        },
        GravityScale(2.0),
        Restitution::coefficient(0.3),
        Friction::coefficient(0.8),
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.5,
        },
        WoodChunk {
            pixels: pixels.clone(),
        },
    ));
}
