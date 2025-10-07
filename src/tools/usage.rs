use bevy::prelude::*;
use super::components::CurrentTool;
use crate::world::PixelWorld;
use crate::physics::components::WoodChunk;

pub fn use_tool(
    current_tool: Res<CurrentTool>,
    mut world: ResMut<PixelWorld>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut chunk_query: Query<(Entity, &Transform, &mut WoodChunk)>,
) {
    // Hold left mouse to use tool
    if !mouse_buttons.pressed(MouseButton::Left) {
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
                    use_tool_at_position(&mut world, &current_tool.tool, pixel_x, pixel_y);

                    // Also break pixels in wood chunks (felled trees)
                    use_tool_on_chunks(&current_tool.tool, world_pos, &mut chunk_query);
                }
            }
        }
    }
}

fn use_tool_at_position(world: &mut PixelWorld, tool: &super::components::Tool, x: i32, y: i32) {
    let tool_radius = 5;

    for dy in -tool_radius..=tool_radius {
        for dx in -tool_radius..=tool_radius {
            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= tool_radius * tool_radius {
                let check_x = x + dx;
                let check_y = y + dy;

                let material = world.get(check_x, check_y);

                // Only destroy if the current tool can break this material
                if tool.can_break(&material) {
                    world.set(check_x, check_y, crate::world::Material::Air);
                }
            }
        }
    }
}

fn use_tool_on_chunks(
    tool: &super::components::Tool,
    world_pos: Vec2,
    chunk_query: &mut Query<(Entity, &Transform, &mut WoodChunk)>,
) {
    use super::components::Tool;

    // Only axe can cut wood chunks
    if *tool != Tool::Axe {
        return;
    }

    let tool_radius = 5.0;

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
        chunk.pixels.retain(|(px, py)| {
            // Transform pixel position to world space (same as rendering)
            let offset_x = (*px - center_x) as f32;
            let offset_y = -(*py - center_y) as f32; // Negative because screen y is flipped

            let rotated_x = offset_x * cos - offset_y * sin;
            let rotated_y = offset_x * sin + offset_y * cos;

            let world_pixel_pos = Vec2::new(
                chunk_pos.x + rotated_x,
                chunk_pos.y + rotated_y,
            );

            // Keep pixel if it's outside tool radius
            let distance = world_pixel_pos.distance(world_pos);
            distance > tool_radius
        });
    }
}
