use bevy::prelude::*;
use super::components::WoodChunk;
use crate::world::Material;

pub fn render_wood_chunks(
    chunk_query: Query<(&WoodChunk, &Transform)>,
    mut gizmos: Gizmos,
) {
    for (chunk, transform) in chunk_query.iter() {
        // Calculate center offset
        let sum_x: i32 = chunk.pixels.iter().map(|(x, _, _)| x).sum();
        let sum_y: i32 = chunk.pixels.iter().map(|(_, y, _)| y).sum();
        let count = chunk.pixels.len() as i32;
        let center_x = sum_x / count;
        let center_y = sum_y / count;

        // Get rotation from transform (z rotation for 2D)
        let rotation_angle = transform.rotation.to_euler(EulerRot::XYZ).2;

        // Draw each pixel as a small rectangle, rotated with the transform
        for (px, py, material) in &chunk.pixels {
            let offset_x = (*px - center_x) as f32;
            let offset_y = -(*py - center_y) as f32; // Negative because screen y is flipped

            // Apply rotation to the offset
            let cos = rotation_angle.cos();
            let sin = rotation_angle.sin();
            let rotated_x = offset_x * cos - offset_y * sin;
            let rotated_y = offset_x * sin + offset_y * cos;

            let world_x = transform.translation.x + rotated_x;
            let world_y = transform.translation.y + rotated_y;

            // Get color based on material
            let color = match material {
                Material::Wood => Color::srgb(0.5, 0.3, 0.15),
                Material::Leaf => Color::srgb(0.2, 0.6, 0.2),
                _ => Color::srgb(0.5, 0.3, 0.15), // Default to wood color
            };

            gizmos.rect_2d(
                Isometry2d::new(Vec2::new(world_x, world_y), Rot2::radians(rotation_angle)),
                Vec2::new(1.0, 1.0),
                color,
            );
        }
    }
}
