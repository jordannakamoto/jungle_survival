use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::components::WoodChunk;

/// Update colliders for wood chunks when their pixels change
pub fn update_chunk_colliders(
    mut chunk_query: Query<(&WoodChunk, &mut Collider), Changed<WoodChunk>>,
) {
    for (chunk, mut collider) in chunk_query.iter_mut() {
        if chunk.pixels.is_empty() {
            continue;
        }

        // Recalculate bounding box
        let min_x = chunk.pixels.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = chunk.pixels.iter().map(|(x, _)| *x).max().unwrap();
        let min_y = chunk.pixels.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = chunk.pixels.iter().map(|(_, y)| *y).max().unwrap();

        let width = (max_x - min_x + 1) as f32;
        let height = (max_y - min_y + 1) as f32;

        // Update the collider to match new size
        *collider = Collider::cuboid(width / 2.0, height / 2.0);
    }
}
