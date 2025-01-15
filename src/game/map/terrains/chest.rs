use rand::Rng;

use crate::assets::Sprite;
use crate::game::map::items::helpers::WOODEN_SPLINTER;
use crate::game::map::terrains::{Dirt, DirtVariant};
use crate::game::map::Passage;
use crate::game::{Item, Terrain, TerrainInteractAction};

use super::super::{TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Chest {
    #[serde(rename = "i")]
    pub items_inside: Vec<Item>,
    #[serde(rename = "o")]
    pub open: bool,
}

impl Chest {
    pub fn new(items_inside: Vec<Item>, open: bool) -> Self {
        Self { items_inside, open }
    }
}

impl TerrainView for Chest {
    fn name(&self) -> &'static str {
        if self.open {
            "chest"
        } else {
            "closed chest"
        }
    }

    fn looks_like(&self) -> Sprite {
        if self.open {
            Sprite::ChestOpen
        } else {
            Sprite::Chest
        }
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Chest {
    fn passage(&self) -> Passage {
        Passage::Passable(50)
    }

    fn can_contain_items(&self) -> bool {
        true
    }

    fn open(&self) -> (Terrain, Vec<Item>) {
        (
            Chest::new(Vec::new(), true).into(),
            self.items_inside.clone(),
        )
    }

    fn close(&self, items: Vec<Item>) -> Terrain {
        Chest::new(items, false).into()
    }

    fn is_smashable(&self) -> bool {
        true
    }

    fn smash_toughness(&self) -> u8 {
        8
    }

    fn smash_result(&self) -> (Terrain, Vec<Item>) {
        let mut rng = rand::thread_rng();
        let dirt_variant = rng.gen::<DirtVariant>();
        let splinters_count = rng.gen_range(1..=3);
        let mut items = self.items_inside.clone();
        for _ in 0..splinters_count {
            items.push(Item::new(WOODEN_SPLINTER));
        }
        (Dirt::new(dirt_variant).into(), items)
    }

    fn supports_action(&self, action: TerrainInteractAction) -> bool {
        match action {
            TerrainInteractAction::Drop => true,
            TerrainInteractAction::Close => self.open,
            TerrainInteractAction::Open => !self.open,
            _ => false,
        }
    }
}
