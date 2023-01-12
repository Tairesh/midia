use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
        Self::new(DirtVariant::Flat)
    }
}

impl TerrainView for Dirt {
    fn name(&self) -> &str {
        match self.variant {
            DirtVariant::Flat => "flat dirt",
            _ => "dirt",
        }
    }

    fn looks_like(&self) -> &'static str {
        match self.variant {
            DirtVariant::LotOfChunks => "dirt1",
            DirtVariant::SomeChunks => "dirt2",
            DirtVariant::Flat => "dirt3",
            DirtVariant::LittleChunks => "dirt4",
            DirtVariant::AlmostNoChunks => "dirt5",
        }
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
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum DirtVariant {
    #[serde(rename = "1")]
    Flat,
    #[serde(rename = "2")]
    LotOfChunks,
    #[serde(rename = "3")]
    SomeChunks,
    #[serde(rename = "4")]
    LittleChunks,
    #[serde(rename = "5")]
    AlmostNoChunks,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        if rng.gen_bool(0.9) {
            DirtVariant::Flat
        } else {
            match rng.gen_range(0..4) {
                0 => DirtVariant::LotOfChunks,
                1 => DirtVariant::SomeChunks,
                2 => DirtVariant::LittleChunks,
                3 => DirtVariant::AlmostNoChunks,
                _ => unreachable!(),
            }
        }
    }
}
