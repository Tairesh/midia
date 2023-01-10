use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

impl TerrainView for Boulder {
    fn name(&self) -> &str {
        match self.size {
            BoulderSize::Huge => "huge boulder",
            BoulderSize::Middle => "boulder",
            BoulderSize::Small => "small boulder",
        }
    }

    fn looks_like(&self) -> &'static str {
        match self.size {
            BoulderSize::Huge => "boulder_huge",
            BoulderSize::Middle => "boulder_middle",
            BoulderSize::Small => "boulder_small",
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
