use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::assets::Sprite;
use crate::game::map::items::helpers::ROCK;
use crate::game::map::terrains::{Dirt, DirtVariant};
use crate::game::traits::{LooksLike, Name};
use crate::game::{Item, Terrain};

use super::super::{Passage, TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Boulder {
    #[serde(rename = "s")]
    size: BoulderSize,
    // TODO: hp, stone type
}

impl Boulder {
    pub fn new(size: BoulderSize) -> Self {
        Self { size }
    }
}

impl Default for Boulder {
    fn default() -> Self {
        Self::new(BoulderSize::Small)
    }
}

impl TerrainView for Boulder {
    fn name(&self) -> &'static str {
        match self.size {
            BoulderSize::Huge => "huge boulder",
            BoulderSize::Middle => "boulder",
            BoulderSize::Small => "small boulder",
        }
    }

    fn looks_like(&self) -> Sprite {
        match self.size {
            BoulderSize::Huge => Sprite::BoulderHuge,
            BoulderSize::Middle => Sprite::Boulder,
            BoulderSize::Small => Sprite::BoulderSmall,
        }
    }

    fn is_transparent(&self) -> bool {
        !matches!(self.size, BoulderSize::Huge)
    }
}

impl TerrainInteract for Boulder {
    fn passage(&self) -> Passage {
        match self.size {
            BoulderSize::Huge | BoulderSize::Middle => Passage::Impassable,
            BoulderSize::Small => Passage::Passable(50.0),
        }
    }

    fn can_stock_items(&self) -> bool {
        false
    }

    fn is_smashable(&self) -> bool {
        true
    }

    fn smash_toughness(&self) -> u8 {
        match self.size {
            BoulderSize::Huge => 12,
            BoulderSize::Middle => 10,
            BoulderSize::Small => 8,
        }
    }

    fn smash_result(&self) -> (Terrain, Vec<Item>) {
        let mut rng = rand::thread_rng();
        let dirt_variant = rng.gen::<DirtVariant>();
        let shards_count = match self.size {
            BoulderSize::Huge => rng.gen_range(3..6),
            BoulderSize::Middle => rng.gen_range(1..3),
            BoulderSize::Small => 1,
        };
        // TODO: add sharp rocks and rubble
        let items = (0..shards_count).map(|_| Item::new(ROCK)).collect();

        (Dirt::new(dirt_variant).into(), items)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum BoulderSize {
    #[serde(rename = "1")]
    Huge,
    #[serde(rename = "2")]
    Middle,
    #[serde(rename = "3")]
    Small,
}

impl Distribution<BoulderSize> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BoulderSize {
        match rng.gen_range(0..3) {
            0 => BoulderSize::Huge,
            1 => BoulderSize::Middle,
            2 => BoulderSize::Small,
            _ => unreachable!(),
        }
    }
}
