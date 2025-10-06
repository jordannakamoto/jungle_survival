use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::{PixelWorld, Material};

pub fn setup_terrain(mut commands: Commands, mut world: ResMut<PixelWorld>) {
    // Create ground from dirt at bottom (y=0 is top, y=600 is bottom)
    world.set_rect(0, 550, 800, 50, Material::Dirt);

    // Create Rapier collider for ground so rigid bodies can collide with it
    // Pixel y=550 corresponds to world y = 300 - 550 = -250
    commands.spawn((
        Transform::from_xyz(0.0, -250.0, 0.0),
        Collider::cuboid(400.0, 25.0),
        RigidBody::Fixed,
    ));

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
