use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Hand,
    Axe,
    Shovel,
}

impl Tool {
    pub fn name(&self) -> &str {
        match self {
            Tool::Hand => "Hand",
            Tool::Axe => "Axe",
            Tool::Shovel => "Shovel",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Tool::Hand => Color::srgb(0.9, 0.8, 0.7),
            Tool::Axe => Color::srgb(0.6, 0.3, 0.1),
            Tool::Shovel => Color::srgb(0.5, 0.5, 0.5),
        }
    }

    /// Check if this tool can break the given material
    pub fn can_break(&self, material: &crate::world::Material) -> bool {
        use crate::world::Material;

        match (self, material) {
            (Tool::Axe, Material::Wood) => true,
            (Tool::Shovel, Material::Dirt) => true,
            (Tool::Shovel, Material::Sand) => true,
            _ => false,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Tool::Hand => Tool::Axe,
            Tool::Axe => Tool::Shovel,
            Tool::Shovel => Tool::Hand,
        }
    }
}

#[derive(Resource)]
pub struct CurrentTool {
    pub tool: Tool,
}

impl Default for CurrentTool {
    fn default() -> Self {
        Self { tool: Tool::Hand }
    }
}

// Resource to track if we're currently grabbing a chunk
#[derive(Resource, Default)]
pub struct GrabbedChunk {
    pub entity: Option<Entity>,
}
