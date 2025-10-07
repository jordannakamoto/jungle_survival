use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::components::WoodChunk;
use std::collections::{HashSet, VecDeque};

/// Detect when chunks have been split and create new rigid bodies for disconnected pieces
pub fn detect_split_chunks(
    mut commands: Commands,
    mut chunk_query: Query<(Entity, &Transform, &mut WoodChunk, &Velocity)>,
) {
    let mut chunks_to_spawn = Vec::new();
    let mut entities_to_despawn = Vec::new();

    for (entity, transform, chunk, velocity) in chunk_query.iter_mut() {
        if chunk.pixels.is_empty() {
            // Mark empty chunks for removal
            entities_to_despawn.push(entity);
            continue;
        }

        // Find all connected components in the chunk
        let components = find_connected_components(&chunk.pixels);

        // If there's more than one component, the chunk has been split
        if components.len() > 1 {
            // Calculate the original chunk's center of mass (before split)
            let original_pixels = &chunk.pixels;
            let sum_x: i32 = original_pixels.iter().map(|(x, _)| x).sum();
            let sum_y: i32 = original_pixels.iter().map(|(_, y)| y).sum();
            let count = original_pixels.len() as i32;
            let original_center_x = sum_x / count;
            let original_center_y = sum_y / count;

            // Store the transform and velocity for spawning new chunks
            let pos = transform.translation;
            let rot = transform.rotation;
            let vel = *velocity;

            // Queue all components to be spawned as new chunks
            for component_pixels in components {
                chunks_to_spawn.push((component_pixels, pos, rot, vel, original_center_x, original_center_y));
            }

            // Mark the original chunk for removal
            entities_to_despawn.push(entity);
        }
    }

    // Despawn old chunks
    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }

    // Spawn new chunks
    for (pixels, pos, rot, vel, orig_center_x, orig_center_y) in chunks_to_spawn {
        spawn_wood_chunk_from_split(&mut commands, pixels, pos, rot, vel, orig_center_x, orig_center_y);
    }
}

/// Find connected components using flood fill
fn find_connected_components(pixels: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    let mut pixel_set: HashSet<(i32, i32)> = pixels.iter().copied().collect();
    let mut components = Vec::new();

    while let Some(&start) = pixel_set.iter().next() {
        let mut component = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        pixel_set.remove(&start);

        while let Some((x, y)) = queue.pop_front() {
            component.push((x, y));

            // Check 4-connected neighbors
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let neighbor = (x + dx, y + dy);
                if pixel_set.remove(&neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        components.push(component);
    }

    components
}

/// Spawn a new wood chunk from split pieces
fn spawn_wood_chunk_from_split(
    commands: &mut Commands,
    pixels: Vec<(i32, i32)>,
    original_pos: Vec3,
    original_rot: Quat,
    original_vel: Velocity,
    original_center_x: i32,
    original_center_y: i32,
) {
    if pixels.is_empty() {
        return;
    }

    // Calculate this piece's center of mass in pixel coordinates
    let sum_x: i32 = pixels.iter().map(|(x, _)| x).sum();
    let sum_y: i32 = pixels.iter().map(|(_, y)| y).sum();
    let count = pixels.len() as i32;
    let piece_center_x = sum_x / count;
    let piece_center_y = sum_y / count;

    // Calculate the offset from the original chunk's center to this piece's center
    let offset_x = (piece_center_x - original_center_x) as f32;
    let offset_y = -((piece_center_y - original_center_y) as f32); // Negative for y-flip

    // Apply the current rotation to this offset
    let rotation_angle = original_rot.to_euler(bevy::math::EulerRot::XYZ).2;
    let cos = rotation_angle.cos();
    let sin = rotation_angle.sin();
    let rotated_offset_x = offset_x * cos - offset_y * sin;
    let rotated_offset_y = offset_x * sin + offset_y * cos;

    // The new position is the original position plus the rotated offset
    let new_pos = Vec3::new(
        original_pos.x + rotated_offset_x,
        original_pos.y + rotated_offset_y,
        original_pos.z,
    );

    // Calculate bounding box for collider
    let min_x = pixels.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = pixels.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = pixels.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = pixels.iter().map(|(_, y)| *y).max().unwrap();

    let width = (max_x - min_x + 1) as f32;
    let height = (max_y - min_y + 1) as f32;

    // Calculate mass based on pixel count
    let mass = pixels.len() as f32 * 0.1;

    info!("Spawning split chunk with {} pixels at position {:?} (offset: {}, {})", pixels.len(), new_pos, rotated_offset_x, rotated_offset_y);

    // Spawn the new chunk at its correct position
    commands.spawn((
        Transform::from_translation(new_pos).with_rotation(original_rot),
        RigidBody::Dynamic,
        Collider::cuboid(width / 2.0, height / 2.0),
        Velocity {
            linvel: original_vel.linvel,
            angvel: original_vel.angvel * 0.8, // Slightly dampen angular velocity
        },
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        GravityScale(1.0),
        ColliderMassProperties::Mass(mass),
        WoodChunk { pixels },
    ));
}
