use crate::assets::Sprite;
use crate::game::traits::{LooksLike, Name};

use super::super::{Passage, TerrainInteract, TerrainView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Pit {}

impl Pit {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Pit {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainView for Pit {
    fn name(&self) -> &'static str {
        "pit"
    }

    fn looks_like(&self) -> Sprite {
        Sprite::Pit
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Pit {
    fn passage(&self) -> Passage {
        Passage::Impassable
    }

    fn can_stock_items(&self) -> bool {
        true
    }
}
