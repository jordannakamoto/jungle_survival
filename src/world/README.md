# World Module

Handles the pixel-based world system, terrain generation, and world interactions.

## Structure

- **pixel_world.rs**: Core pixel grid data structure and rendering
- **materials.rs**: Material types (Wood, Dirt, Sand, Leaf, Fiber, Air) with properties
- **terrain.rs**: Procedural terrain generation with trees and ground
- **service.rs**: WorldService facade for coordinate conversions and common operations
- **particles.rs**: Particle system for visual effects on material interactions
- **ground_colliders.rs**: Rapier physics collider generation from terrain
- **digging.rs**: Legacy digging system (may be deprecated)

## Key Resources

- `PixelWorld`: Main world grid (800x600 pixels by default)
- `WorldService`: Provides coordinate conversion and world manipulation helpers
- `ParticleSpawnEvent`: Event for decoupled particle spawning

## Coordinate System

World uses two coordinate systems:
- **Pixel coordinates**: (0,0) at top-left, (799, 599) at bottom-right
- **World coordinates**: (0,0) at center, using standard Cartesian coordinates

Use `WorldService::pixel_to_world()` and `WorldService::world_to_pixel()` for conversions.

## Usage Example

```rust
// Get material at world position
let material = WorldService::get_material_at_world(&world, world_pos);

// Break blocks in radius
let broken = WorldService::break_blocks_in_radius(&mut world, world_pos, 5.0);

// Spawn particles
particle_events.send(ParticleSpawnEvent {
    position: world_pos,
    material: Material::Wood,
});
```
