/// World dimensions in pixels
pub const WORLD_PIXEL_WIDTH: usize = 800;
pub const WORLD_PIXEL_HEIGHT: usize = 600;

/// Coordinate conversion constants
/// World coordinates center (0, 0) at the middle of the pixel grid
pub const PIXEL_TO_WORLD_OFFSET_X: f32 = 400.0;
pub const PIXEL_TO_WORLD_OFFSET_Y: f32 = 300.0;

/// Physics constants
pub const CHUNK_DETECTION_INTERVAL: f32 = 0.1; // seconds

/// Rendering constants
pub const PIXEL_SIZE: f32 = 1.0;

// Player movement constants
// Note: MAX_SLOPE_HEIGHT is defined in player/movement.rs (currently 6 pixels)
// This allows the player to automatically climb slopes up to 6 pixels high per frame
