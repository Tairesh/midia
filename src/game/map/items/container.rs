use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::game_data::AmmoType;
use crate::game::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub items: Vec<Item>,
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

        if item.proto().stackable {
            for existing_item in &mut self.items {
                if existing_item.proto().id == item.proto().id {
                    existing_item.increase_stack(item.stack_size());
                    return;
                }
            }
        }

        if self.free_volume() == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::map::items::helpers::{BOOK, LAZULI};

    #[test]
    fn test_container_push_item() {
        let mut container = Container::new(Vec::new(), 3, HashSet::new());
        let book1 = Item::new(BOOK).with_named("Book of Testing");
        let book2 = Item::new(BOOK).with_named("Another Book of Testing");
        let lazuli = Item::new(LAZULI).with_stack(5);

        container.push_item(book1);
        container.push_item(book2);
        container.push_item(lazuli);

        assert_eq!(container.items.len(), 3);
        assert_eq!(container.volume_used(), 3);
        assert_eq!(container.free_volume(), 0);

        // Try to add another item when container is full
        let book3 = Item::new(BOOK).with_named("Bad Book");
        container.push_item(book3);
        assert_eq!(container.items.len(), 3); // Should not increase

        // Test stackable item
        let another_lazuli = Item::new(LAZULI).with_stack(3);
        container.push_item(another_lazuli);

        assert_eq!(container.items.len(), 3); // Still 3 items
        let stackable = container
            .items
            .iter()
            .find(|i| i.proto().id == LAZULI)
            .unwrap();
        assert_eq!(stackable.stack_size(), 8); // Stack size should be updated
    }
}
