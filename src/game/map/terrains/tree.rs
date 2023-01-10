use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::super::{Passage, TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tree {
    #[serde(rename = "v")]
    variant: TreeVariant,
    // TODO: hp
}

impl Tree {
    pub fn new(variant: TreeVariant) -> Self {
        Self { variant }
    }
}

impl TerrainView for Tree {
    fn name(&self) -> &str {
        "dead tree"
    }

    fn looks_like(&self) -> &'static str {
        match self.variant {
            TreeVariant::DeadTree => "dead_tree",
            TreeVariant::DeadPine => "dead_pine",
            TreeVariant::DeadHickory => "dead_hickory",
        }
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl TerrainInteract for Tree {
    fn passage(&self) -> Passage {
        Passage::Impassable
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
#[non_exhaustive]
#[allow(clippy::enum_variant_names)]
pub enum TreeVariant {
    #[serde(rename = "1")]
    DeadTree,
    #[serde(rename = "2")]
    DeadPine,
    #[serde(rename = "3")]
    DeadHickory,
}

impl Distribution<TreeVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TreeVariant {
        match rng.gen_range(0..3) {
            0 => TreeVariant::DeadTree,
            1 => TreeVariant::DeadPine,
            2 => TreeVariant::DeadHickory,
            _ => unreachable!(),
        }
    }
}
