use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::{PixelWorld, Material};

#[derive(Component)]
pub struct GroundCollider;

#[derive(Resource)]
pub struct GroundColliderTimer {
    pub timer: Timer,
}

impl Default for GroundColliderTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

/// System to regenerate ground colliders based on current terrain
pub fn update_ground_colliders(
    mut commands: Commands,
    world: Res<PixelWorld>,
    existing_colliders: Query<Entity, With<GroundCollider>>,
    time: Res<Time>,
    mut timer: ResMut<GroundColliderTimer>,
) {
    // Only update every 0.5 seconds to avoid performance issues
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    // Remove all existing ground colliders
    let count = existing_colliders.iter().count();
    for entity in existing_colliders.iter() {
        commands.entity(entity).despawn();
    }

    info!("Removed {} existing ground colliders, regenerating...", count);

    // Generate new colliders from the current terrain
    // We'll scan for horizontal runs of solid ground (dirt/sand)
    generate_ground_colliders(&mut commands, &world);
}

fn generate_ground_colliders(commands: &mut Commands, world: &PixelWorld) {
    let width = world.width as i32;
    let height = world.height as i32;

    // For each column, find the topmost solid pixel (the surface)
    let mut surface_points: Vec<Vec2> = vec![];

    for x in (0..width).step_by(5) {  // Sample every 5 pixels for performance
        // Scan from top to bottom to find first solid pixel
        for y in 0..height {
            if matches!(world.get(x, y), Material::Dirt | Material::Sand) {
                // Convert pixel coordinates to world coordinates
                let world_x = x as f32 - 400.0;
                let world_y = 300.0 - y as f32;
                surface_points.push(Vec2::new(world_x, world_y));
                break;
            }
        }
    }

    // Create polyline colliders that precisely follow the terrain
    // Split into segments to avoid single massive collider
    let max_segment_length = 150;
    let mut segment_count = 0;

    for chunk_start in (0..surface_points.len()).step_by(max_segment_length) {
        let chunk_end = (chunk_start + max_segment_length).min(surface_points.len());

        if chunk_end - chunk_start < 2 {
            continue;
        }

        let segment = &surface_points[chunk_start..chunk_end];

        // Create a polyline collider from the points
        let indices: Vec<[u32; 2]> = (0..segment.len() - 1)
            .map(|i| [i as u32, (i + 1) as u32])
            .collect();

        if !indices.is_empty() {
            commands.spawn((
                Transform::from_xyz(0.0, 0.0, 0.0),
                Collider::polyline(segment.to_vec(), Some(indices)),
                RigidBody::Fixed,
                GroundCollider,
            ));
            segment_count += 1;
        }
    }

    info!("Generated {} ground colliders with {} surface points", segment_count, surface_points.len());
}
