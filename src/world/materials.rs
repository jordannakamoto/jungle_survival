use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Material {
    Air,
    Dirt,
    Wood,
    Sand,
}

impl Material {
    pub fn color(&self) -> Color {
        match self {
            Material::Air => Color::srgba(0.1, 0.1, 0.15, 0.0),
            Material::Dirt => Color::srgb(0.4, 0.3, 0.2),
            Material::Wood => Color::srgb(0.5, 0.3, 0.15),
            Material::Sand => Color::srgb(0.8, 0.7, 0.5),
        }
    }

    pub fn is_solid(&self) -> bool {
        !matches!(self, Material::Air)
    }

    pub fn density(&self) -> u8 {
        match self {
            Material::Air => 0,
            Material::Sand => 2,
            Material::Dirt => 3,
            Material::Wood => 5,
        }
    }
}
