use rand::Rng;

use crate::game::map::terrains::{Dirt, DirtVariant};
use crate::game::map::Passage;
use crate::game::{Item, Terrain};

use super::super::{TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Chest {
    #[serde(rename = "i")]
    items_inside: Vec<Item>,
    #[serde(rename = "o")]
    open: bool,
}

impl Chest {
    pub fn new(items_inside: Vec<Item>, open: bool) -> Self {
        Self { items_inside, open }
    }
}

impl TerrainView for Chest {
    fn name(&self) -> &str {
        "chest"
    }

    fn looks_like(&self) -> &'static str {
        if self.open {
            "opened_chest"
        } else {
            "chest"
        }
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Chest {
    fn passage(&self) -> Passage {
        Passage::Passable(50.0)
    }

    fn can_stock_items(&self) -> bool {
        self.open
    }

    fn can_be_opened(&self) -> bool {
        !self.open
    }

    fn can_be_closed(&self) -> bool {
        self.open
    }

    fn can_suck_items_on_close(&self) -> bool {
        true
    }

    fn open(&self) -> (Terrain, Vec<Item>) {
        (
            Chest::new(Vec::new(), true).into(),
            self.items_inside.clone(),
        )
    }

    fn close_and_suck_items(&self, items: Vec<Item>) -> Terrain {
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
        let splinters_count = rng.gen_range(1..=3);
        let mut items = self.items_inside.clone();
        for _ in 0..splinters_count {
            items.push(Item::new("wooden_splinter"));
        }
        (Dirt::new(DirtVariant::Flat).into(), items)
    }
}
