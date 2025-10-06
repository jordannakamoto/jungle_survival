use bevy::prelude::*;
use rand::Rng;
use super::materials::Material;

#[derive(Resource)]
pub struct PixelWorld {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Material>,
    dirty_chunks: Vec<bool>,
    chunk_size: usize,
}

impl PixelWorld {
    pub fn new(width: usize, height: usize) -> Self {
        let chunk_size = 32;
        let num_chunks = (width / chunk_size) * (height / chunk_size);

        Self {
            width,
            height,
            pixels: vec![Material::Air; width * height],
            dirty_chunks: vec![true; num_chunks],
            chunk_size,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Material {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return Material::Air;
        }
        self.pixels[y as usize * self.width + x as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, material: Material) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let idx = y as usize * self.width + x as usize;
        self.pixels[idx] = material;
        self.mark_chunk_dirty(x, y);
    }

    fn mark_chunk_dirty(&mut self, x: i32, y: i32) {
        let chunk_x = x as usize / self.chunk_size;
        let chunk_y = y as usize / self.chunk_size;
        let chunks_wide = self.width / self.chunk_size;
        let chunk_idx = chunk_y * chunks_wide + chunk_x;
        if chunk_idx < self.dirty_chunks.len() {
            self.dirty_chunks[chunk_idx] = true;
        }
    }

    pub fn set_rect(&mut self, x: i32, y: i32, w: i32, h: i32, material: Material) {
        for dy in 0..h {
            for dx in 0..w {
                self.set(x + dx, y + dy, material);
            }
        }
    }

    pub fn set_circle(&mut self, cx: i32, cy: i32, radius: i32, material: Material) {
        let r_sq = radius * radius;
        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy <= r_sq {
                    self.set(x, y, material);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct PixelRenderer {
    image_handle: Handle<Image>,
}

pub fn setup_renderer(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    world: Res<PixelWorld>,
) {
    // Create image for rendering pixels
    let image = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: world.width as u32,
            height: world.height as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD | bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    let image_handle = images.add(image);

    commands.spawn((
        Sprite::from_image(image_handle.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PixelRenderer { image_handle: image_handle.clone() },
    ));
}

pub fn update_pixels(mut world: ResMut<PixelWorld>) {
    let mut rng = rand::thread_rng();
    let width = world.width;
    let height = world.height;

    // Only update every other frame to reduce CPU usage
    static mut FRAME_SKIP: u32 = 0;
    unsafe {
        FRAME_SKIP += 1;
        if FRAME_SKIP % 2 != 0 {
            return;
        }
    }

    // Create a copy for reading while we write
    let old_pixels = world.pixels.clone();

    // Scan from bottom to top for better sand settling performance
    for y in (0..height - 1).rev() {
        for x in 0..width {
            let idx = y * width + x;
            let material = old_pixels[idx];

            match material {
                Material::Sand => {
                    // Sand falls down
                    let below = world.get(x as i32, y as i32 + 1);
                    if below == Material::Air {
                        world.set(x as i32, y as i32, Material::Air);
                        world.set(x as i32, y as i32 + 1, Material::Sand);
                    } else if below.density() < material.density() {
                        // Fall through lighter materials
                        world.set(x as i32, y as i32, below);
                        world.set(x as i32, y as i32 + 1, Material::Sand);
                    } else {
                        // Try diagonal
                        let dir = if rng.gen_bool(0.5) { -1 } else { 1 };
                        let diag = world.get(x as i32 + dir, y as i32 + 1);
                        if diag == Material::Air {
                            world.set(x as i32, y as i32, Material::Air);
                            world.set(x as i32 + dir, y as i32 + 1, Material::Sand);
                        }
                    }
                }
                Material::Wood | Material::Dirt => {
                    // Solid materials don't move
                }
                Material::Air => {}
            }
        }
    }
}

pub fn render_pixels(
    world: Res<PixelWorld>,
    query: Query<&PixelRenderer>,
    mut images: ResMut<Assets<Image>>,
) {
    let Ok(renderer) = query.single() else {
        return;
    };

    // Only re-render if the world has changed
    if !world.is_changed() {
        return;
    }

    if let Some(image) = images.get_mut(&renderer.image_handle) {
        if let Some(data) = &mut image.data {
            // Update image pixels (y=0 at top, increases downward)
            for y in 0..world.height {
                for x in 0..world.width {
                    let material = world.get(x as i32, y as i32);
                    let color = material.color();
                    let idx = (y * world.width + x) * 4;

                    data[idx] = (color.to_srgba().red * 255.0) as u8;
                    data[idx + 1] = (color.to_srgba().green * 255.0) as u8;
                    data[idx + 2] = (color.to_srgba().blue * 255.0) as u8;
                    data[idx + 3] = (color.to_srgba().alpha * 255.0) as u8;
                }
            }
        }
    }
}
