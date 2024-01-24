use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use tetra::graphics::Color;

use crate::colors::Colors;
use crate::game::traits::{LooksLike, Name};

use super::super::{terrains::Pit, Item, Passage, Terrain, TerrainInteract, TerrainView};

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

    fn looks_like(&self) -> &'static str {
        match self.variant {
            DirtVariant::Dirt1 => "dirt1",
            DirtVariant::Dirt2 => "dirt2",
            DirtVariant::Dirt3 => "dirt3",
            DirtVariant::Dirt4 => "dirt4",
            DirtVariant::Dirt5 => "dirt5",
            DirtVariant::Dirt6 => "dirt6",
            DirtVariant::Dirt7 => "dirt7",
            DirtVariant::Dirt8 => "dirt8",
            DirtVariant::Dirt9 => "dirt9",
            DirtVariant::Dirt10 => "dirt10",
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
        Passage::Passable(10.0)
    }

    fn is_diggable(&self) -> bool {
        true
    }

    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        (Pit::new().into(), vec![])
    }

    fn can_stock_items(&self) -> bool {
        true
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

#[cfg(test)]
mod tests {
    use super::{Dirt, DirtVariant, Terrain, TerrainInteract, TerrainView};

    #[test]
    fn test_dirt() {
        let terrain: Terrain = Dirt::new(DirtVariant::Dirt1).into();
        assert_eq!("dirt", terrain.name());
        assert!(terrain.is_diggable());
    }
}
