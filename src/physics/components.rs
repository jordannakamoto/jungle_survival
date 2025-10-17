use bevy::prelude::*;
use crate::world::Material;

#[derive(Component)]
pub struct WoodChunk {
    pub pixels: Vec<(i32, i32, Material)>,
}
