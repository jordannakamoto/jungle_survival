# UI Module

Handles user interface elements and displays.

## Structure

- **mod.rs**: UIPlugin
- **tool_indicator.rs**: Shows current active tool on screen

## Current Features

- Tool indicator text (displays "Tool: Hand/Axe/Shovel")

## Design

UI module is well-isolated from game logic - it only reads from game resources (like `CurrentTool`) and doesn't modify game state. This makes it easy to extend with new UI elements.

## Future Extensions

Could add:
- Health/stamina bars
- Inventory display
- Minimap
- Debug overlays
