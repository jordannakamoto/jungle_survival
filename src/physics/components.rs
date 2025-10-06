use bevy::prelude::*;

#[derive(Component)]
pub struct WoodChunk {
    pub pixels: Vec<(i32, i32)>,
}
