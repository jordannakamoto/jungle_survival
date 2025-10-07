use bevy::prelude::*;
use super::{PixelWorld, Material};

pub fn setup_terrain(mut world: ResMut<PixelWorld>) {
    // Create ground from dirt at bottom (y=0 is top, y=600 is bottom)
    // Dirt occupies pixel y from 550 to 600 (50 pixels tall)
    world.set_rect(0, 550, 800, 50, Material::Dirt);

    // Ground colliders are now generated dynamically by ground_colliders system
    // This allows them to update when terrain is dug

    // Spawn trees made of wood pixels
    for i in 0..6 {
        let x = 100 + i * 120;
        let trunk_height = 80;

        // Tree trunk - vertical line of wood (growing upward from ground at y=550)
        world.set_rect(x, 550 - trunk_height, 15, trunk_height, Material::Wood);

        // Tree foliage - circle of wood at top (lower y = higher on screen)
        world.set_circle(x + 7, 550 - trunk_height - 20, 25, Material::Wood);
    }

    // Add some sand piles for testing
    world.set_circle(400, 400, 20, Material::Sand);
}
