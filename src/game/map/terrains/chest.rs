use crate::game::map::Passage;
use crate::game::{Item, Terrain};

use super::super::{TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Chest {
    #[serde(rename = "i")]
    pub items: Vec<Item>,
    #[serde(rename = "o")]
    pub open: bool,
}

impl Chest {
    pub fn new(items: Vec<Item>, open: bool) -> Self {
        Self { items, open }
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

    fn open(&self) -> Terrain {
        Chest::new(self.items.clone(), true).into()
    }

    fn close(&self) -> Terrain {
        Chest::new(self.items.clone(), false).into()
    }
}
