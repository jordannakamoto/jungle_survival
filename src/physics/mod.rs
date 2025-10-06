pub mod components;
pub mod chunk_detection;
pub mod chunk_rendering;

use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            chunk_detection::detect_floating_chunks,
            chunk_rendering::render_wood_chunks,
        ));
    }
}
