# Physics Module

Handles dynamic physics simulation for falling trees and wood chunks using Rapier2D.

## Structure

- **components.rs**: `WoodChunk` component for tracking pixels in physics bodies
- **chunk_detection.rs**: Detects floating wood and converts to rigid bodies
- **chunk_splitting.rs**: Splits large chunks when they break apart
- **chunk_rendering.rs**: Renders wood chunks as colored pixels
- **collider_update.rs**: Updates colliders when chunks change

## How It Works

1. **Detection**: Every 0.1s, scans for wood pixels not connected to ground (dirt/sand)
2. **Conversion**: Floating wood groups become Rapier rigid bodies with realistic physics
3. **Tree Falling**: Tall structures get angular velocity to tip over naturally
4. **Rendering**: Wood chunks render their pixels with proper rotation/translation

## Key Components

- `WoodChunk`: Stores pixel positions for a physics body
- Uses `WorldService` for coordinate conversions between pixels and world space

## Integration

Physics bodies can be interacted with by tools (see tools module).
When wood chunks are cut or destroyed, their pixels are tracked and removed appropriately.
