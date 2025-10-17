# Tools Module

Implements the tool system for interacting with the world and physics objects.

## Structure

- **components.rs**: Tool definitions (Hand, Axe, Shovel) and material breaking rules
- **usage.rs**: Core tool usage logic, block breaking, and particle spawning
- **switching.rs**: Tool switching input handling
- **hand.rs**: Hand tool specific behavior

## Tool Types

- **Hand**: Can break Leaf, Fiber materials
- **Axe**: Can break Wood, Leaf, Fiber (cuts trees!)
- **Shovel**: Can break Dirt, Sand, Leaf, Fiber

## Architecture

Tools interact with both:
1. **PixelWorld**: Static terrain blocks
2. **WoodChunks**: Dynamic physics bodies (fallen trees)

Uses event-based particle spawning via `ParticleSpawnEvent` to decouple visual effects.

## Key Resources

- `CurrentTool`: Tracks active tool
- `ParticleSpawnTimer`: Throttles particle generation
- `BlockBreakTimer`: Controls block breaking rate

## Usage Pattern

```rust
// Tool checks mouse input
// Converts cursor to world position via WorldService
// Breaks blocks in radius
// Emits ParticleSpawnEvent for visual feedback
```
