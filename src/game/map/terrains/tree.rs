use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::super::{Passage, TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Tree {
    #[serde(rename = "v")]
    variant: TreeVariant,
}

impl Tree {
    pub fn new(variant: TreeVariant) -> Self {
        Self { variant }
    }
}

impl TerrainView for Tree {
    fn name(&self) -> &str {
        "tree"
    }

    fn looks_like(&self) -> &'static str {
        match self.variant {
            TreeVariant::DeadTree => "tree_dead",
            TreeVariant::DeadPine => "pine_dead",
            TreeVariant::DeadHickory => "hickory_dead",
            TreeVariant::DeadWillow => "willow_dead",
            TreeVariant::DeadBirch => "birch_dead",
            TreeVariant::Tree => "tree",
            TreeVariant::Pine => "pine",
            TreeVariant::Hickory => "hickory",
            TreeVariant::Willow => "willow",
            TreeVariant::Birch => "birch",
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

    fn can_stock_items(&self) -> bool {
        false
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
    #[serde(rename = "4")]
    DeadWillow,
    #[serde(rename = "5")]
    DeadBirch,
    #[serde(rename = "6")]
    Tree,
    #[serde(rename = "7")]
    Pine,
    #[serde(rename = "8")]
    Hickory,
    #[serde(rename = "9")]
    Willow,
    #[serde(rename = "10")]
    Birch,
}

impl Distribution<TreeVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TreeVariant {
        match rng.gen_range(0..10) {
            0 => TreeVariant::DeadTree,
            1 => TreeVariant::DeadPine,
            2 => TreeVariant::DeadHickory,
            3 => TreeVariant::DeadWillow,
            4 => TreeVariant::DeadBirch,
            5 => TreeVariant::Tree,
            6 => TreeVariant::Pine,
            7 => TreeVariant::Hickory,
            8 => TreeVariant::Willow,
            9 => TreeVariant::Birch,
            _ => unreachable!(),
        }
    }
}
