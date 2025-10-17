use bevy::prelude::*;
use super::materials::Material;
use super::pixel_world::PixelWorld;
use crate::constants::{PIXEL_TO_WORLD_OFFSET_X, PIXEL_TO_WORLD_OFFSET_Y, WORLD_PIXEL_WIDTH, WORLD_PIXEL_HEIGHT};

/// Service layer for world operations that provides a facade over PixelWorld.
/// This centralizes coordinate conversions and common world manipulation patterns.
#[derive(Resource)]
pub struct WorldService;

impl WorldService {
    /// Convert pixel coordinates to world coordinates
    /// World coordinates have (0, 0) at the center of the screen
    pub fn pixel_to_world(pixel_x: i32, pixel_y: i32) -> Vec2 {
        Vec2::new(
            pixel_x as f32 - PIXEL_TO_WORLD_OFFSET_X,
            PIXEL_TO_WORLD_OFFSET_Y - pixel_y as f32,
        )
    }

    /// Convert world coordinates to pixel coordinates
    /// Returns None if the position is outside the world bounds
    pub fn world_to_pixel(world_pos: Vec2) -> (i32, i32) {
        let pixel_x = (world_pos.x + PIXEL_TO_WORLD_OFFSET_X) as i32;
        let pixel_y = (PIXEL_TO_WORLD_OFFSET_Y - world_pos.y) as i32;
        (pixel_x, pixel_y)
    }

    /// Check if world coordinates are within bounds
    pub fn is_in_bounds(world_pos: Vec2) -> bool {
        let (px, py) = Self::world_to_pixel(world_pos);
        px >= 0 && px < WORLD_PIXEL_WIDTH as i32 && py >= 0 && py < WORLD_PIXEL_HEIGHT as i32
    }

    /// Get material at world position
    pub fn get_material_at_world(world: &PixelWorld, world_pos: Vec2) -> Material {
        let (px, py) = Self::world_to_pixel(world_pos);
        world.get(px, py)
    }

    /// Set material at world position
    pub fn set_material_at_world(world: &mut PixelWorld, world_pos: Vec2, material: Material) {
        let (px, py) = Self::world_to_pixel(world_pos);
        world.set(px, py, material);
    }

    /// Break blocks in a circle around a world position
    /// Returns the materials that were broken
    pub fn break_blocks_in_radius(
        world: &mut PixelWorld,
        world_pos: Vec2,
        radius: f32,
    ) -> Vec<(Material, Vec2)> {
        let (center_px, center_py) = Self::world_to_pixel(world_pos);
        let radius_pixels = radius as i32;
        let r_sq = radius_pixels * radius_pixels;

        let mut broken_materials = Vec::new();

        for py in (center_py - radius_pixels)..=(center_py + radius_pixels) {
            for px in (center_px - radius_pixels)..=(center_px + radius_pixels) {
                let dx = px - center_px;
                let dy = py - center_py;

                if dx * dx + dy * dy <= r_sq {
                    let material = world.get(px, py);
                    if material != Material::Air {
                        let material_world_pos = Self::pixel_to_world(px, py);
                        broken_materials.push((material, material_world_pos));
                        world.set(px, py, Material::Air);
                    }
                }
            }
        }

        broken_materials
    }

    /// Check if there's a solid collision at world position (for player physics)
    pub fn has_collision_at(world: &PixelWorld, world_pos: Vec2) -> bool {
        let material = Self::get_material_at_world(world, world_pos);
        material != Material::Air
    }

    /// Get all pixels in a rectangular region (in pixel coordinates)
    /// Returns vector of (x, y, material) tuples
    pub fn get_pixels_in_rect(
        world: &PixelWorld,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> Vec<(i32, i32, Material)> {
        let mut pixels = Vec::new();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let material = world.get(x, y);
                if material != Material::Air {
                    pixels.push((x, y, material));
                }
            }
        }

        pixels
    }

    /// Remove pixels in a specific set of pixel coordinates
    /// Returns the materials that were removed
    pub fn remove_pixels(
        world: &mut PixelWorld,
        pixels: &[(i32, i32)],
    ) -> Vec<Material> {
        let mut materials = Vec::new();

        for (x, y) in pixels {
            let material = world.get(*x, *y);
            if material != Material::Air {
                materials.push(material);
                world.set(*x, *y, Material::Air);
            }
        }

        materials
    }

    /// Set a circle of material at world position
    pub fn set_circle_at_world(
        world: &mut PixelWorld,
        world_pos: Vec2,
        radius: f32,
        material: Material,
    ) {
        let (center_px, center_py) = Self::world_to_pixel(world_pos);
        world.set_circle(center_px, center_py, radius as i32, material);
    }
}
