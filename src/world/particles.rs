use bevy::prelude::*;
use rand::Rng;
use super::materials::Material;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ParticleSpawnEvent>()
            .add_systems(Update, (
                handle_particle_spawn_events,
                update_particles,
                cleanup_dead_particles,
            ));
    }
}

/// Event for requesting particle spawns
/// This decouples particle spawning from the systems that trigger them
#[derive(Event)]
pub struct ParticleSpawnEvent {
    pub position: Vec2,
    pub material: Material,
}

/// Small visual particle that doesn't interact with physics
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// Parameters for spawning particles when interacting with materials
#[derive(Clone)]
pub struct MaterialInteractionParams {
    /// Number of particles to spawn per interaction
    pub particle_count_range: (u32, u32),
    /// Speed range for particles
    pub speed_range: (f32, f32),
    /// How much randomness in direction (0.0 = straight up, 1.0 = all directions)
    pub spread: f32,
    /// Particle lifetime range in seconds
    pub lifetime_range: (f32, f32),
    /// Particle size range
    pub size_range: (f32, f32),
    /// Gravity strength for particles
    pub gravity: f32,
}

impl Default for MaterialInteractionParams {
    fn default() -> Self {
        Self {
            particle_count_range: (3, 8),
            speed_range: (30.0, 80.0),
            spread: 0.8,
            lifetime_range: (0.3, 0.8),
            size_range: (1.0, 3.0),
            gravity: 150.0,
        }
    }
}

impl MaterialInteractionParams {
    /// Get interaction parameters based on material type
    pub fn for_material(material: Material) -> Self {
        match material {
            Material::Dirt => Self {
                particle_count_range: (2, 5), // Reduced from (5, 12)
                speed_range: (20.0, 60.0),
                spread: 0.9,
                lifetime_range: (0.3, 0.6), // Reduced from (0.4, 1.0)
                size_range: (1.5, 3.0), // Reduced max from 3.5
                gravity: 200.0,
            },
            Material::Wood => Self {
                particle_count_range: (2, 4), // Reduced from (3, 8)
                speed_range: (40.0, 100.0),
                spread: 0.7,
                lifetime_range: (0.3, 0.7), // Reduced from (0.5, 1.2)
                size_range: (1.0, 2.0), // Reduced max from 2.5
                gravity: 120.0,
            },
            Material::Sand => Self {
                particle_count_range: (3, 6), // Reduced from (8, 15)
                speed_range: (15.0, 50.0),
                spread: 1.0,
                lifetime_range: (0.2, 0.5), // Reduced from (0.3, 0.7)
                size_range: (0.8, 1.5), // Reduced max from 2.0
                gravity: 250.0,
            },
            Material::Leaf => Self {
                particle_count_range: (3, 7),
                speed_range: (10.0, 40.0),
                spread: 1.0,
                lifetime_range: (0.4, 0.8),
                size_range: (1.0, 2.5),
                gravity: 80.0, // Very light, floats more
            },
            Material::Fiber => Self {
                particle_count_range: (4, 8),
                speed_range: (15.0, 45.0),
                spread: 0.8,
                lifetime_range: (0.3, 0.7),
                size_range: (1.0, 2.0),
                gravity: 100.0, // Light but heavier than leaves
            },
            Material::Air => Self::default(),
        }
    }
}

/// System that handles particle spawn events
fn handle_particle_spawn_events(
    mut commands: Commands,
    mut events: EventReader<ParticleSpawnEvent>,
) {
    for event in events.read() {
        let params = MaterialInteractionParams::for_material(event.material);
        spawn_material_particles(&mut commands, event.position, event.material, &params);
    }
}

/// Spawn particles at a position when interacting with material
pub fn spawn_material_particles(
    commands: &mut Commands,
    position: Vec2,
    material: Material,
    params: &MaterialInteractionParams,
) {
    if material == Material::Air {
        return;
    }

    let mut rng = rand::thread_rng();
    let particle_count = rng.gen_range(params.particle_count_range.0..=params.particle_count_range.1);

    // Get base color with some variation
    let base_color = material.color();

    for _ in 0..particle_count {
        // Random direction with upward bias and spread
        let angle = std::f32::consts::FRAC_PI_2 // Start at 90 degrees (straight up)
            + rng.gen_range(-std::f32::consts::PI * params.spread..std::f32::consts::PI * params.spread);

        let speed = rng.gen_range(params.speed_range.0..params.speed_range.1);
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

        let lifetime = rng.gen_range(params.lifetime_range.0..params.lifetime_range.1);
        let size = rng.gen_range(params.size_range.0..params.size_range.1);

        // Add some color variation
        let color_variance = 0.15;
        let color = Color::srgb(
            (base_color.to_srgba().red + rng.gen_range(-color_variance..color_variance)).clamp(0.0, 1.0),
            (base_color.to_srgba().green + rng.gen_range(-color_variance..color_variance)).clamp(0.0, 1.0),
            (base_color.to_srgba().blue + rng.gen_range(-color_variance..color_variance)).clamp(0.0, 1.0),
        );

        // Small random offset from spawn position
        let offset = Vec2::new(
            rng.gen_range(-3.0..3.0),
            rng.gen_range(-3.0..3.0),
        );

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            Transform::from_translation(position.extend(1.0) + offset.extend(0.0)),
            Particle {
                velocity,
                lifetime,
                max_lifetime: lifetime,
            },
        ));
    }
}

/// Update particle physics (simple velocity + gravity, no collision)
fn update_particles(
    mut particle_query: Query<(&mut Transform, &mut Particle)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut particle) in particle_query.iter_mut() {
        // Update lifetime
        particle.lifetime -= dt;

        // Apply velocity
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;

        // Get gravity from params (we'll store it in the velocity update)
        // For now, use a default gravity value
        let gravity = 200.0;
        particle.velocity.y -= gravity * dt;
    }
}

/// Remove particles that have expired
fn cleanup_dead_particles(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &Particle, &mut Sprite)>,
) {
    for (entity, particle, mut sprite) in particle_query.iter_mut() {
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Fade out particles near the end of their life
            let alpha = (particle.lifetime / particle.max_lifetime).clamp(0.0, 1.0);
            if alpha < 1.0 {
                sprite.color = sprite.color.with_alpha(alpha);
            }
        }
    }
}
