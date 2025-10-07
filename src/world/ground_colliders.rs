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
    let mut collider_count = 0;

    // For each column, find the topmost solid pixel (the surface)
    // Then group adjacent surface pixels into horizontal segments
    let mut surface_y: Vec<Option<i32>> = vec![None; width as usize];

    for x in 0..width {
        // Scan from top to bottom to find first solid pixel
        for y in 0..height {
            if matches!(world.get(x, y), Material::Dirt | Material::Sand) {
                surface_y[x as usize] = Some(y);
                break;
            }
        }
    }

    // Now create colliders for continuous horizontal segments at the same height
    let mut segment_start: Option<i32> = None;
    let mut segment_y: Option<i32> = None;

    for x in 0..width {
        if let Some(y) = surface_y[x as usize] {
            match (segment_start, segment_y) {
                (None, None) => {
                    // Start new segment
                    segment_start = Some(x);
                    segment_y = Some(y);
                }
                (Some(_), Some(prev_y)) if prev_y == y => {
                    // Continue segment (same height)
                    continue;
                }
                (Some(start_x), Some(prev_y)) => {
                    // Height changed, finish previous segment
                    if x - start_x > 2 { // Minimum width of 2 pixels
                        create_ground_collider(commands, start_x, x, prev_y);
                        collider_count += 1;
                    }
                    // Start new segment
                    segment_start = Some(x);
                    segment_y = Some(y);
                }
                _ => {}
            }
        } else {
            // No surface here, finish segment if exists
            if let (Some(start_x), Some(prev_y)) = (segment_start, segment_y) {
                if x - start_x > 2 {
                    create_ground_collider(commands, start_x, x, prev_y);
                    collider_count += 1;
                }
            }
            segment_start = None;
            segment_y = None;
        }
    }

    // Handle segment that extends to the end
    if let (Some(start_x), Some(y)) = (segment_start, segment_y) {
        if width - start_x > 2 {
            create_ground_collider(commands, start_x, width, y);
            collider_count += 1;
        }
    }

    info!("Generated {} ground colliders", collider_count);
}

fn create_ground_collider(commands: &mut Commands, start_x: i32, end_x: i32, pixel_y: i32) {
    let center_x = (start_x + end_x) as f32 / 2.0;
    let width = (end_x - start_x) as f32;

    // Convert pixel coordinates to world coordinates
    let world_x = center_x - 400.0;
    let world_y = 300.0 - pixel_y as f32;

    // Create a thin horizontal collider
    commands.spawn((
        Transform::from_xyz(world_x, world_y, 0.0),
        Collider::cuboid(width / 2.0, 5.0), // 5 pixel half-height
        RigidBody::Fixed,
        GroundCollider,
    ));
}
