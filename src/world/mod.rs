pub mod pixel_world;
pub mod materials;
pub mod terrain;
pub mod digging;
pub mod ground_colliders;
pub mod particles;
pub mod service;

pub use pixel_world::PixelWorld;
pub use materials::Material;
pub use particles::ParticleSpawnEvent;
pub use service::WorldService;

use bevy::prelude::*;
use crate::constants::{WORLD_PIXEL_WIDTH, WORLD_PIXEL_HEIGHT};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PixelWorld::new(WORLD_PIXEL_WIDTH, WORLD_PIXEL_HEIGHT))
            .insert_resource(WorldService)
            .init_resource::<ground_colliders::GroundColliderTimer>()
            .add_plugins(particles::ParticlePlugin)
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
