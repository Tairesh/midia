use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::assets::Sprite;

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
    fn name(&self) -> &'static str {
        match self.variant {
            TreeVariant::DeadTree => "dead tree",
            TreeVariant::DeadPine => "dead pine",
            TreeVariant::DeadHickory => "dead hickory",
            TreeVariant::DeadWillow => "dead willow",
            TreeVariant::DeadBirch => "dead birch",
            TreeVariant::Tree => "tree",
            TreeVariant::Pine => "pine",
            TreeVariant::Hickory => "hickory",
            TreeVariant::Willow => "willow",
            TreeVariant::Birch => "birch",
        }
    }

    fn looks_like(&self) -> Sprite {
        match self.variant {
            TreeVariant::DeadTree => Sprite::TreeDead,
            TreeVariant::DeadPine => Sprite::PineDead,
            TreeVariant::DeadHickory => Sprite::HickoryDead,
            TreeVariant::DeadWillow => Sprite::WillowDead,
            TreeVariant::DeadBirch => Sprite::BirchDead,
            TreeVariant::Tree => Sprite::Tree,
            TreeVariant::Pine => Sprite::Pine,
            TreeVariant::Hickory => Sprite::Hickory,
            TreeVariant::Willow => Sprite::Willow,
            TreeVariant::Birch => Sprite::Birch,
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

pub struct DeadTrees;

pub struct LiveTrees;

impl Distribution<TreeVariant> for DeadTrees {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TreeVariant {
        match rng.gen_range(0..5) {
            0 => TreeVariant::DeadTree,
            1 => TreeVariant::DeadPine,
            2 => TreeVariant::DeadHickory,
            3 => TreeVariant::DeadWillow,
            4 => TreeVariant::DeadBirch,
            _ => unreachable!(),
        }
    }
}

impl Distribution<TreeVariant> for LiveTrees {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TreeVariant {
        match rng.gen_range(0..5) {
            0 => TreeVariant::Tree,
            1 => TreeVariant::Pine,
            2 => TreeVariant::Hickory,
            3 => TreeVariant::Willow,
            4 => TreeVariant::Birch,
            _ => unreachable!(),
        }
    }
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
