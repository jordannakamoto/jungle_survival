# Player Module

Handles player character movement, physics, and rendering.

## Structure

- **components.rs**: `Player` resource with position and velocity
- **movement.rs**: Movement physics with gravity, jumping, collision detection
- **rendering.rs**: Player sprite rendering

## Movement System

- **Gravity**: 600.0 pixels/s² downward acceleration
- **Speed**: 150.0 pixels/s horizontal movement
- **Jump**: 300.0 pixels/s upward impulse (only when grounded)

## Collision

Player uses pixel-perfect collision with the world:
- Only collides with `Material::Dirt` (can pass through trees, sand)
- Checks ground, ceiling, and wall collisions independently
- Position is in pixel coordinates

## Controls

Movement handled through `GameInput` resource (see input module):
- Arrow keys for movement
- Up arrow for jump (when on ground)
