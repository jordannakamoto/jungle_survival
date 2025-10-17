# Debug Module

Interactive developer menu system with persistent settings.

## Usage

Press **`** (backtick/tilde key) to toggle the debug menu.

## Features

### Developer Menu
- **Player Collision**: Shows player bounding box and collision check points
- **World Collision**: Visualizes dirt collision areas (sampled for performance)
- **Physics Colliders**: Shows physics body outlines for fallen trees
- **FPS Display**: Shows frames per second in top-right corner

### Persistent Settings
- All debug settings are saved to `debug_config.json`
- Settings persist between game sessions
- Config file is gitignored

### Expandable
This menu is designed to be easily extended with new debug features:
1. Add a new field to `DebugConfig` struct
2. Add a new variant to `DebugSetting` enum
3. Add toggle logic in `DebugConfig::toggle()`
4. Add button to the menu in `draw_debug_menu()`
5. Implement visualization in `draw_collision_debug()` or create new system

## Technical Details

- Uses Bevy's Gizmos API for rendering debug overlays
- Click-based UI with hover states
- Coordinates converted from screen space to world space for button detection
- Menu positioned at top-left of screen
