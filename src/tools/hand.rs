use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::components::{CurrentTool, Tool, GrabbedChunk};
use crate::physics::components::WoodChunk;

pub fn handle_hand_tool(
    current_tool: Res<CurrentTool>,
    mut grabbed_chunk: ResMut<GrabbedChunk>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut chunk_query: Query<(Entity, &Transform, &mut Velocity), With<WoodChunk>>,
) {
    // Only use hand when Hand tool is selected
    if current_tool.tool != Tool::Hand {
        // Release any grabbed chunk if we switch tools
        if grabbed_chunk.entity.is_some() {
            grabbed_chunk.entity = None;
        }
        return;
    }

    // Get mouse position in world
    let world_pos = if let Ok(window) = windows.single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera_query.single() {
                if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    Some(ray.origin.truncate())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if let Some(world_pos) = world_pos {
        // On mouse press, try to grab a chunk
        if mouse_buttons.just_pressed(MouseButton::Left) && grabbed_chunk.entity.is_none() {
            // Find closest chunk within grab distance
            let grab_distance = 50.0;
            let mut closest_chunk: Option<(Entity, f32)> = None;

            for (entity, transform, _) in chunk_query.iter() {
                let distance = transform.translation.truncate().distance(world_pos);
                if distance < grab_distance {
                    if let Some((_, closest_dist)) = closest_chunk {
                        if distance < closest_dist {
                            closest_chunk = Some((entity, distance));
                        }
                    } else {
                        closest_chunk = Some((entity, distance));
                    }
                }
            }

            if let Some((entity, _)) = closest_chunk {
                grabbed_chunk.entity = Some(entity);
            }
        }

        // While holding, move the chunk with mouse
        if mouse_buttons.pressed(MouseButton::Left) {
            if let Some(entity) = grabbed_chunk.entity {
                if let Ok((_, transform, mut velocity)) = chunk_query.get_mut(entity) {
                    // Move chunk towards mouse position with velocity
                    let current_pos = transform.translation.truncate();
                    let direction = world_pos - current_pos;

                    // Use a strong spring-like force to move the chunk
                    let strength = 20.0;
                    velocity.linvel = direction * strength;

                    // Dampen rotation when grabbed
                    velocity.angvel *= 0.9;
                }
            }
        }

        // Rotate held object with Q/E keys
        if let Some(entity) = grabbed_chunk.entity {
            if let Ok((_, _, mut velocity)) = chunk_query.get_mut(entity) {
                // Fine rotation on tap (just_pressed), continuous when held
                if keyboard.just_pressed(KeyCode::KeyQ) {
                    velocity.angvel = -0.5; // Small impulse for fine rotation
                } else if keyboard.just_pressed(KeyCode::KeyE) {
                    velocity.angvel = 0.5; // Small impulse for fine rotation
                } else if keyboard.pressed(KeyCode::KeyQ) {
                    velocity.angvel = -2.0; // Continuous rotation when held
                } else if keyboard.pressed(KeyCode::KeyE) {
                    velocity.angvel = 2.0; // Continuous rotation when held
                }
            }
        }

        // On mouse release, stop grabbing
        if mouse_buttons.just_released(MouseButton::Left) {
            grabbed_chunk.entity = None;
        }
    }
}
