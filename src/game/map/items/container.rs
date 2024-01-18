use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::game_data::AmmoType;
use crate::game::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    items: Vec<Item>,
    pub max_volume: u8,
    for_ammo: HashSet<AmmoType>,
}

impl Container {
    pub fn new(
        items: impl Into<Vec<Item>>,
        max_volume: u8,
        for_ammo: impl Into<HashSet<AmmoType>>,
    ) -> Self {
        Self {
            items: items.into(),
            max_volume,
            for_ammo: for_ammo.into(),
        }
    }

    pub fn volume_used(&self) -> u8 {
        self.items.len() as u8
    }

    pub fn free_volume(&self) -> u8 {
        self.max_volume - self.volume_used()
    }

    pub fn is_for_ammo(&self) -> bool {
        !self.for_ammo.is_empty()
    }

    pub fn push_item(&mut self, item: Item) {
        if self.is_for_ammo() && !self.for_ammo.iter().any(|t| item.is_ammo(*t)) {
            return;
        }

        self.items.push(item);
    }

    pub fn push_items(&mut self, items: impl Into<Vec<Item>>) {
        let items: Vec<Item> = items.into();
        for item in items {
            self.push_item(item);
        }
    }
}
