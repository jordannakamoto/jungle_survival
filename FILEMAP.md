# Jungle Survival - File Structure

## Overview
This is a Noita-style pixel physics survival game built with Bevy and Rust. The codebase is organized into clear, modular systems for scalability.

## Directory Structure

```
jungle_survival/
├── Cargo.toml                 # Project dependencies and metadata
├── Cargo.lock                 # Locked dependency versions
├── FILEMAP.md                 # This file - comprehensive code organization guide
├── README.md                  # Project documentation (to be created)
└── src/
    ├── main.rs                # Entry point - initializes Bevy app and plugins
    │
    ├── player/                # Player system - completely decoupled from world
    │   ├── mod.rs             # Player plugin registration
    │   ├── components.rs      # Player resource/component definitions
    │   ├── movement.rs        # Player physics and collision detection
    │   └── rendering.rs       # Player visual representation
    │
    ├── world/                 # Environment and pixel simulation
    │   ├── mod.rs             # World plugin registration
    │   ├── materials.rs       # Material types (Air, Wood, Dirt, Sand)
    │   ├── pixel_world.rs     # Pixel grid storage and cellular automata
    │   └── terrain.rs         # World generation (ground, trees, etc.)
    │
    ├── physics/               # Rigid body physics for disconnected chunks
    │   ├── mod.rs             # Physics plugin registration
    │   ├── components.rs      # WoodChunk and physics components
    │   ├── chunk_detection.rs # Flood fill algorithm to find floating chunks
    │   └── chunk_rendering.rs # Render rigid body chunks with rotation
    │
    ├── input/                 # Input abstraction layer
    │   └── mod.rs             # GameInput resource and key mapping
    │
    └── ui/                    # Future: menus, HUD, etc.
        └── mod.rs             # (To be implemented)
```

## Module Responsibilities

### `main.rs`
- Initialize Bevy engine
- Register all plugins in correct order
- Configure window settings
- Set up Rapier physics engine

### `player/`
**Fully decoupled from world systems - operates independently**

- **components.rs**: Player state (position, velocity, size)
- **movement.rs**: Movement physics, collision detection with pixel world
- **rendering.rs**: Draw player sprite/gizmo

**Key feature**: Player uses pixel world for collision but doesn't modify it

### `world/`
**Manages the pixel-based environment**

- **materials.rs**: Enum defining all material types and their properties
- **pixel_world.rs**:
  - 800x600 pixel grid storage
  - Cellular automata updates (sand falling, etc.)
  - Pixel manipulation (get/set/set_rect/set_circle)
  - Rendering pixel world to texture
- **terrain.rs**: Initial world setup (ground, trees, decorations)

### `physics/`
**Converts pixel structures into rigid bodies**

- **components.rs**: WoodChunk component that stores original pixel positions
- **chunk_detection.rs**:
  - Flood fill to find wood connected to ground
  - Detect disconnected chunks
  - Convert to rigid bodies with realistic physics
  - Calculate fall direction based on center of mass
- **chunk_rendering.rs**: Render chunks with proper rotation

### `input/`
**Abstraction layer between raw input and game actions**

- Converts keyboard/mouse into high-level actions (movement, dig, etc.)
- Makes it easy to rebind controls or add gamepad support
- **GameInput resource**: movement: Vec2, interact: bool, chop: bool, etc.

## Data Flow

### Player Movement
```
Input → GameInput → movement.rs → Player position update
                           ↓
                    PixelWorld (collision check only)
```

### Tree Chopping
```
Mouse click → tree_chopping → Destroy pixels in PixelWorld
                                        ↓
                            chunk_detection (every 0.5s)
                                        ↓
                              Flood fill disconnected wood
                                        ↓
                              Spawn rigid body chunks
                                        ↓
                            Rapier physics simulation
```

### Pixel Simulation
```
update_pixels → For each pixel type:
                  - Sand: fall down/diagonal
                  - Wood/Dirt: stay static
                  - Air: empty space
                      ↓
                render_pixels → Draw to texture
```

## Key Design Principles

1. **Separation of Concerns**: Player, World, and Physics are independent systems
2. **Modular**: Each system can be modified without affecting others
3. **Scalable**: Easy to add new materials, player abilities, or physics behaviors
4. **Performance**: Chunk-based updates, only active areas simulated
5. **Type Safety**: Strong typing with enums and components

## Future Expansion Points

- `ui/`: Menus, inventory, crafting system
- `world/biomes.rs`: Different terrain types
- `world/structures.rs`: Buildings, caves
- `player/inventory.rs`: Item management
- `player/abilities.rs`: Special actions
- `physics/liquids.rs`: Water/lava physics
- `entities/`: Mobs, NPCs
- `multiplayer/`: Networking code

## Technical Stack

- **Bevy 0.16**: Game engine
- **bevy_rapier2d 0.31.0**: 2D physics
- **rand 0.8**: Random number generation
- **Rust 2021**: Programming language

## Build & Run

```bash
cargo build --release  # Optimized build
cargo run              # Development run
```

## Performance Notes

- Pixel world: 800x600 = 480,000 pixels
- Chunk system: Only updates dirty 32x32 chunks
- Physics detection: Runs every 0.5 seconds
- Target: 60 FPS on modern hardware
