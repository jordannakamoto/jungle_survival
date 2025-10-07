pub mod pixel_world;
pub mod materials;
pub mod terrain;
pub mod digging;
pub mod ground_colliders;

pub use pixel_world::PixelWorld;
pub use materials::Material;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PixelWorld::new(800, 600))
            .init_resource::<ground_colliders::GroundColliderTimer>()
            .add_systems(Startup, (
                pixel_world::setup_renderer,
                terrain::setup_terrain,
            ))
            .add_systems(Update, (
                pixel_world::update_pixels,
                pixel_world::render_pixels,
                ground_colliders::update_ground_colliders,
            ).chain());
    }
}
