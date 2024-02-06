use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::colors::Colors;

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Material {
    Cloth,
    Wool,
    Leather,
    Wood,
    Stone,
    Iron,
    Steel,
    Obsidian,
    Demonite,
    LapisLazuli,
    Bone,
    Flesh,
    Plant,
    Paper,
}

impl Material {
    pub fn is_hard(self) -> bool {
        matches!(
            self,
            Self::Wood
                | Self::Stone
                | Self::Iron
                | Self::Steel
                | Self::Obsidian
                | Self::Demonite
                | Self::LapisLazuli
                | Self::Bone
        )
    }
}

impl From<Material> for Color {
    fn from(value: Material) -> Self {
        match value {
            Material::Cloth | Material::Wool => Colors::CLOTH,
            Material::Leather => Colors::LEATHER,
            Material::Wood => Colors::WOOD,
            Material::Stone => Colors::STONE,
            Material::Iron | Material::Steel => Colors::METAL,
            Material::Obsidian => Colors::OBSIDIAN,
            Material::Demonite => Colors::DEMONITE,
            Material::LapisLazuli => Colors::LAPIS_LAZULI,
            Material::Bone => Colors::BONE,
            Material::Flesh => Colors::RED,
            Material::Plant => Colors::PLANT,
            Material::Paper => Colors::LIGHT_SEPIA,
        }
    }
}
