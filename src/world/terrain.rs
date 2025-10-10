use bevy::prelude::*;
use rand::Rng;
use super::{PixelWorld, Material};

pub fn setup_terrain(mut world: ResMut<PixelWorld>) {
    // Create ground from dirt at bottom (y=0 is top, y=600 is bottom)
    // Dirt occupies pixel y from 550 to 600 (50 pixels tall)
    world.set_rect(0, 550, 800, 50, Material::Dirt);

    // Ground colliders are now generated dynamically by ground_colliders system
    // This allows them to update when terrain is dug

    // Spawn palm trees with varied sizes
    let mut rng = rand::thread_rng();
    for i in 0..6 {
        let x = 100 + i * 120;

        // Random trunk height (60-120 pixels)
        let trunk_height = rng.gen_range(60..=120);

        // Random trunk width (8-14 pixels)
        let trunk_width = rng.gen_range(8..=14);

        // Palm trunk - narrower and taller than regular trees
        world.set_rect(x, 550 - trunk_height, trunk_width, trunk_height, Material::Wood);

        // Palm fronds - simple leaf crown at the top
        let top_y = 550 - trunk_height;
        let center_x = x + trunk_width / 2;

        // Random frond size (20-30 pixels)
        let frond_length = rng.gen_range(20..=30);

        // Create 5-7 simple fronds radiating from the top
        let num_fronds = rng.gen_range(5..=7);
        for j in 0..num_fronds {
            let angle = (j as f32 / num_fronds as f32) * std::f32::consts::TAU;
            let angle_offset = rng.gen_range(-0.3..0.3); // Randomness

            // Draw each frond as a simple curved shape
            for dist in 0..frond_length {
                let progress = dist as f32 / frond_length as f32;

                // Gentle downward curve
                let curve = progress * progress * 0.4;

                let frond_x = center_x + (angle.cos() * dist as f32) as i32;
                let frond_y = top_y - 5 + ((angle.sin() + angle_offset) * dist as f32 * curve) as i32;

                // Width tapers from base to tip (wider in middle)
                let width_factor = 1.0 - (progress - 0.5).abs() * 2.0; // Peak at middle
                let width = ((6.0 * width_factor) as i32).max(1);

                // Draw the frond
                world.set_rect(frond_x - width / 2, frond_y, width, 2, Material::Leaf);
            }
        }

        // Add coconuts - small circles near the top of the trunk
        if rng.gen_bool(0.7) { // 70% chance to have coconuts
            let num_coconuts = rng.gen_range(2..=4);
            for _ in 0..num_coconuts {
                let coconut_x = center_x + rng.gen_range(-8..=8);
                let coconut_y = top_y + rng.gen_range(0..10);
                world.set_circle(coconut_x, coconut_y, rng.gen_range(3..=5), Material::Wood);
            }
        }
    }

    // Add some sand piles for testing
    world.set_circle(400, 400, 20, Material::Sand);

    // Spawn fiber bushes scattered on the ground
    for _ in 0..15 {
        let bush_x = rng.gen_range(50..750);
        let bush_y = 550; // Ground level

        // Random bush size
        let bush_width = rng.gen_range(15..=25);
        let bush_height = rng.gen_range(12..=20);

        // Create a rounded bush shape
        for dy in 0..bush_height {
            for dx in 0..bush_width {
                let center_x = bush_width / 2;
                let center_y = bush_height / 2;

                // Distance from center
                let dist_x = (dx as i32 - center_x as i32).abs();
                let dist_y = (dy as i32 - center_y as i32).abs();

                // Create oval/round bush shape
                let normalized_dist = (dist_x * dist_x) as f32 / (center_x * center_x) as f32
                    + (dist_y * dist_y) as f32 / (center_y * center_y) as f32;

                // Add some randomness to edges for organic look
                let threshold = rng.gen_range(0.8..1.2);

                if normalized_dist <= threshold {
                    world.set(
                        bush_x + dx as i32,
                        bush_y - dy as i32,
                        Material::Fiber
                    );
                }
            }
        }
    }
}
