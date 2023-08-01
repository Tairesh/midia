use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::game_data::AmmoType;
use crate::game::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub items: Vec<Item>,
    pub max_volume: u8,
    pub for_ammo: HashSet<AmmoType>,
}

impl Container {
    pub fn is_for_ammo(&self) -> bool {
        !self.for_ammo.is_empty()
    }

    pub fn push_item(&mut self, item: Item) {
        if self.is_for_ammo() && !self.for_ammo.iter().any(|t| item.is_ammo(*t)) {
            return;
        }

        self.items.push(item);
    }
}
