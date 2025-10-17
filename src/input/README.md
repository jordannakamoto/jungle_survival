# Input Module

Provides an abstraction layer over Bevy's raw input system for semantic game actions.

## Structure

- **mod.rs**: `InputPlugin` and `GameInput` resource

## Purpose

Decouples game logic from specific key bindings by providing a `GameInput` resource that other systems can read. This makes it easy to:
- Change control schemes
- Add controller support
- Implement key remapping

## Current Mapping

- Arrow keys → `movement.x` and `movement.y`
- Can be easily extended for other actions

## Usage

```rust
fn my_system(game_input: Res<GameInput>) {
    if game_input.movement.x > 0.0 {
        // Player wants to move right
    }
}
```

Other systems should read `GameInput` rather than querying raw keyboard/mouse state directly.
