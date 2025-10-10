use bevy::prelude::*;
use rand::Rng;
use super::components::CurrentTool;
use crate::world::{PixelWorld, Material, MaterialInteractionParams, spawn_material_particles};
use crate::physics::components::WoodChunk;

/// Cooldown timer to prevent spawning too many particles
#[derive(Resource)]
pub struct ParticleSpawnTimer {
    timer: Timer,
}

impl Default for ParticleSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating), // Spawn particles max 10 times per second
        }
    }
}

/// Cooldown timer for actually breaking blocks (much slower for trees)
#[derive(Resource)]
pub struct BlockBreakTimer {
    timer: Timer,
}

impl Default for BlockBreakTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating), // Break blocks 2 times per second
        }
    }
}

pub fn use_tool(
    mut commands: Commands,
    current_tool: Res<CurrentTool>,
    mut world: ResMut<PixelWorld>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut chunk_query: Query<(Entity, &Transform, &mut WoodChunk)>,
    mut particle_timer: ResMut<ParticleSpawnTimer>,
    mut break_timer: ResMut<BlockBreakTimer>,
    time: Res<Time>,
) {
    // Hold left mouse to use tool
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    // Update timers
    particle_timer.timer.tick(time.delta());
    break_timer.timer.tick(time.delta());

    let should_spawn_particles = particle_timer.timer.just_finished();
    let should_break_blocks = break_timer.timer.just_finished();

    // Only proceed if we can break blocks this frame
    if !should_break_blocks {
        return;
    }

    // Get mouse position in world
    if let Ok(window) = windows.single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera_query.single() {
                if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    let world_pos = ray.origin.truncate();
                    let pixel_x = (world_pos.x + 400.0) as i32;
                    let pixel_y = (300.0 - world_pos.y) as i32;

                    // Use tool to break blocks in the pixel world
                    use_tool_at_position(&mut commands, &mut world, &current_tool.tool, pixel_x, pixel_y, world_pos, should_spawn_particles);

                    // Also break pixels in wood chunks (felled trees)
                    use_tool_on_chunks(&mut commands, &current_tool.tool, world_pos, &mut chunk_query, should_spawn_particles);
                }
            }
        }
    }
}

fn use_tool_at_position(
    commands: &mut Commands,
    world: &mut PixelWorld,
    tool: &super::components::Tool,
    x: i32,
    y: i32,
    _world_pos: Vec2,
    should_spawn_particles: bool,
) {
    let mut rng = rand::thread_rng();

    // Random smaller radius (1-3 pixels instead of fixed 5)
    let tool_radius = rng.gen_range(1..=3);
    let mut broken_materials: Vec<(Material, Vec2)> = Vec::new();

    for dy in -tool_radius..=tool_radius {
        for dx in -tool_radius..=tool_radius {
            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= tool_radius * tool_radius {
                let check_x = x + dx;
                let check_y = y + dy;

                let material = world.get(check_x, check_y);

                // Only destroy if the current tool can break this material
                // Add 30% chance to miss individual pixels for more randomness
                if tool.can_break(&material) && rng.gen_bool(0.7) {
                    world.set(check_x, check_y, Material::Air);

                    // Collect broken material for particle spawning
                    if should_spawn_particles {
                        let pixel_world_pos = Vec2::new(
                            check_x as f32 - 400.0,
                            300.0 - check_y as f32,
                        );
                        broken_materials.push((material, pixel_world_pos));
                    }
                }
            }
        }
    }

    // Spawn particles semi-randomly (not for every pixel, but in clusters)
    if should_spawn_particles && !broken_materials.is_empty() {
        // Group broken materials by type
        let mut material_groups: std::collections::HashMap<Material, Vec<Vec2>> = std::collections::HashMap::new();
        for (material, pos) in broken_materials {
            material_groups.entry(material).or_insert_with(Vec::new).push(pos);
        }

        // Spawn particle effects for each material type
        for (material, positions) in material_groups {
            // Choose 1-2 random positions from this material type to spawn particles (reduced from 2-4)
            let num_particle_spawns = rng.gen_range(1.min(positions.len())..=2.min(positions.len()));

            let params = MaterialInteractionParams::for_material(material);

            for _ in 0..num_particle_spawns {
                if let Some(pos) = positions.get(rng.gen_range(0..positions.len())) {
                    spawn_material_particles(commands, *pos, material, &params);
                }
            }
        }
    }
}

fn use_tool_on_chunks(
    commands: &mut Commands,
    tool: &super::components::Tool,
    world_pos: Vec2,
    chunk_query: &mut Query<(Entity, &Transform, &mut WoodChunk)>,
    should_spawn_particles: bool,
) {
    use super::components::Tool;

    // Only axe can cut wood chunks
    if *tool != Tool::Axe {
        return;
    }

    let mut rng = rand::thread_rng();
    // Random smaller radius (1.0-3.0 pixels instead of fixed 5.0)
    let tool_radius = rng.gen_range(1.0..=3.0);
    let mut removed_positions: Vec<Vec2> = Vec::new();

    for (_entity, transform, mut chunk) in chunk_query.iter_mut() {
        // Calculate center offset (same as rendering)
        let sum_x: i32 = chunk.pixels.iter().map(|(x, _)| x).sum();
        let sum_y: i32 = chunk.pixels.iter().map(|(_, y)| y).sum();
        let count = chunk.pixels.len() as i32;
        let center_x = sum_x / count;
        let center_y = sum_y / count;

        let chunk_pos = transform.translation.truncate();
        let rotation = transform.rotation.to_euler(bevy::math::EulerRot::XYZ).2;
        let cos = rotation.cos();
        let sin = rotation.sin();

        // Check each pixel in the chunk
        let original_len = chunk.pixels.len();
        let mut new_pixels = Vec::new();

        for (px, py) in chunk.pixels.iter() {
            // Transform pixel position to world space (same as rendering)
            let offset_x = (*px - center_x) as f32;
            let offset_y = -(*py - center_y) as f32; // Negative because screen y is flipped

            let rotated_x = offset_x * cos - offset_y * sin;
            let rotated_y = offset_x * sin + offset_y * cos;

            let world_pixel_pos = Vec2::new(
                chunk_pos.x + rotated_x,
                chunk_pos.y + rotated_y,
            );

            // Keep pixel if it's outside tool radius or random miss (30% chance)
            let distance = world_pixel_pos.distance(world_pos);
            let in_radius = distance <= tool_radius;
            let should_remove = in_radius && rng.gen_bool(0.7); // 70% chance to actually remove

            if should_remove {
                // If we're removing this pixel, save its position for particles
                if should_spawn_particles {
                    removed_positions.push(world_pixel_pos);
                }
            } else {
                // Keep this pixel
                new_pixels.push((*px, *py));
            }
        }

        chunk.pixels = new_pixels;

        // Spawn particles if we removed any pixels from this chunk
        let removed_count = original_len - chunk.pixels.len();
        if should_spawn_particles && removed_count > 0 {
            // Spawn particles at 1-2 random removed positions (reduced from 2-4)
            let num_particle_spawns = rng.gen_range(1.min(removed_positions.len())..=2.min(removed_positions.len()));
            let params = MaterialInteractionParams::for_material(Material::Wood);

            for _ in 0..num_particle_spawns {
                if let Some(pos) = removed_positions.get(rng.gen_range(0..removed_positions.len())) {
                    spawn_material_particles(commands, *pos, Material::Wood, &params);
                }
            }
        }
    }
}
