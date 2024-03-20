use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use tetra::graphics::Color;

use crate::assets::Sprite;
use crate::colors::Colors;
use crate::game::TerrainInteractAction;

use super::super::{Passage, Terrain, TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Dirt {
    #[serde(rename = "v")]
    variant: DirtVariant,
}

impl Dirt {
    pub fn new(variant: DirtVariant) -> Self {
        Self { variant }
    }
}

impl Default for Dirt {
    fn default() -> Self {
        Self::new(DirtVariant::Dirt2)
    }
}

impl TerrainView for Dirt {
    fn name(&self) -> &'static str {
        "dirt"
    }

    fn looks_like(&self) -> Sprite {
        match self.variant {
            DirtVariant::Dirt1 => Sprite::Dirt1,
            DirtVariant::Dirt2 => Sprite::Dirt2,
            DirtVariant::Dirt3 => Sprite::Dirt3,
            DirtVariant::Dirt4 => Sprite::Dirt4,
            DirtVariant::Dirt5 => Sprite::Dirt5,
            DirtVariant::Dirt6 => Sprite::Dirt6,
            DirtVariant::Dirt7 => Sprite::Dirt7,
            DirtVariant::Dirt8 => Sprite::Dirt8,
            DirtVariant::Dirt9 => Sprite::Dirt9,
            DirtVariant::Dirt10 => Sprite::Dirt10,
        }
    }

    fn color(&self) -> Option<Color> {
        Some(Colors::DIRT)
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Dirt {
    fn passage(&self) -> Passage {
        Passage::Passable(10)
    }

    fn on_step(&self) -> Option<Terrain> {
        if self.variant == DirtVariant::Dirt2 {
            None
        } else if rand::thread_rng().gen_bool(0.1) {
            Some(Dirt::new(DirtVariant::Dirt2).into())
        } else {
            None
        }
    }

    fn supports_action(&self, action: TerrainInteractAction) -> bool {
        action == TerrainInteractAction::Drop
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DirtVariant {
    #[serde(rename = "1")]
    Dirt1,
    #[serde(rename = "2")]
    Dirt2,
    #[serde(rename = "3")]
    Dirt3,
    #[serde(rename = "4")]
    Dirt4,
    #[serde(rename = "5")]
    Dirt5,
    #[serde(rename = "6")]
    Dirt6,
    #[serde(rename = "7")]
    Dirt7,
    #[serde(rename = "8")]
    Dirt8,
    #[serde(rename = "9")]
    Dirt9,
    #[serde(rename = "10")]
    Dirt10,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        match rng.gen_range(0..10) {
            0 => DirtVariant::Dirt1,
            1 => DirtVariant::Dirt2,
            2 => DirtVariant::Dirt3,
            3 => DirtVariant::Dirt4,
            4 => DirtVariant::Dirt5,
            5 => DirtVariant::Dirt6,
            6 => DirtVariant::Dirt7,
            7 => DirtVariant::Dirt8,
            8 => DirtVariant::Dirt9,
            9 => DirtVariant::Dirt10,
            _ => unreachable!(),
        }
    }
}
