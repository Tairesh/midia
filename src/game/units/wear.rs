use crate::game::{BodySlot, Item};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Wear {
    items: Vec<(Item, usize)>,
}

impl Wear {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn can_add(&self, item: &Item, variant: usize) -> bool {
        if let Some(wearable) = &item.proto().wearable {
            let layer = wearable.layer;
            let slots = &wearable.variants[variant];
            !self
                .items
                .iter()
                .filter(|(i, _)| i.proto().wearable.as_ref().unwrap().layer == layer)
                .any(|(i, v)| {
                    let variant = &i.proto().wearable.as_ref().unwrap().variants[*v];
                    variant.iter().any(|s| slots.contains(s))
                })
        } else {
            false
        }
    }

    pub fn add(&mut self, item: Item, variant: usize) {
        self.items.push((item, variant));
    }

    pub fn take_all(&mut self) -> Vec<Item> {
        self.items.drain(..).map(|(item, _)| item).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.items.iter().map(|(item, _)| item)
    }

    pub fn get_items_by_slot(&self, slot: BodySlot) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|(item, variant)| {
                if let Some(wearable) = &item.proto().wearable {
                    wearable.variants[*variant].contains(&slot)
                } else {
                    false
                }
            })
            .map(|(item, _)| item)
            .collect()
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Default for Wear {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{BACKPACK, CLOAK, HAT, LEATHER_ARM_GUARD, RAGS};
    use crate::game::Item;

    use super::Wear;

    #[test]
    fn test_cant_wear_two_items_in_one_slot() {
        let mut wear = Wear::new();
        wear.add(Item::new(CLOAK), 0);
        assert!(!wear.can_add(&Item::new(RAGS), 0));
    }

    #[test]
    fn test_can_wear_two_items_in_different_slots() {
        let mut wear = Wear::new();
        wear.add(Item::new(CLOAK), 0);
        assert!(wear.can_add(&Item::new(HAT), 0));
    }

    #[test]
    fn test_can_wear_two_items_in_different_layers() {
        let mut wear = Wear::new();
        wear.add(Item::new(CLOAK), 0);
        assert!(wear.can_add(&Item::new(BACKPACK), 0));
    }

    #[test]
    fn test_can_wear_two_oversleeves_in_different_slots() {
        let mut wear = Wear::new();
        wear.add(Item::new(CLOAK), 0);
        let oversleeve = Item::new(LEATHER_ARM_GUARD);
        assert!(wear.can_add(&oversleeve, 0));
        wear.add(oversleeve.clone(), 0);
        assert!(wear.can_add(&oversleeve, 1));
        wear.add(oversleeve.clone(), 1);
        assert!(!wear.can_add(&oversleeve, 0));
        assert!(!wear.can_add(&oversleeve, 1));
    }
}
