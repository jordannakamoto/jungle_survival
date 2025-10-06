use bevy::prelude::*;
use super::{PixelWorld, Material};
use crate::input::GameInput;
use crate::player::components::Player;

pub struct DiggingPlugin;

impl Plugin for DiggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dig_system);
    }
}

/// Universal digging/destruction system - destroys any solid pixels
fn dig_system(
    _player: Res<Player>,
    mut world: ResMut<PixelWorld>,
    _game_input: Res<GameInput>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    // Hold left mouse to dig
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

                    // Destroy any solid material in a circle
                    dig_at_position(&mut world, pixel_x, pixel_y);
                }
            }
        }
    }
}

fn dig_at_position(world: &mut PixelWorld, x: i32, y: i32) {
    let dig_radius = 5;

    for dy in -dig_radius..=dig_radius {
        for dx in -dig_radius..=dig_radius {
            let dist_sq = dx * dx + dy * dy;
            if dist_sq <= dig_radius * dig_radius {
                let check_x = x + dx;
                let check_y = y + dy;

                // Destroy any solid material (wood, dirt, sand, etc.)
                if world.get(check_x, check_y).is_solid() {
                    world.set(check_x, check_y, Material::Air);
                }
            }
        }
    }
}
