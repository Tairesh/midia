use std::collections::HashSet;

use crate::game::{AmmoType, BodySlot, Item};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Wear {
    items: Vec<(Item, usize)>,
}

impl Wear {
    pub fn new(items: impl Into<Vec<(Item, usize)>>) -> Self {
        Self {
            items: items.into(),
        }
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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Item> {
        self.items.iter_mut().map(|(item, _)| item)
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

    pub fn has_ammo(&self, ammo_types: &HashSet<AmmoType>) -> bool {
        ammo_types
            .iter()
            .any(|&ammo_type| self.iter().any(|wear| wear.has_ammo(ammo_type)))
    }

    pub fn get_ammo(&self, ammo_types: &HashSet<AmmoType>) -> Option<&Item> {
        for &ammo_type in ammo_types {
            for item in self.iter() {
                if let Some(container) = item.container() {
                    let index = container.items.iter().position(|i| i.is_ammo(ammo_type));
                    if let Some(index) = index {
                        return Some(&container.items[index]);
                    }
                }
            }
        }

        None
    }

    pub fn remove_ammo(&mut self, ammo_types: &HashSet<AmmoType>) -> Option<Item> {
        for &ammo_type in ammo_types {
            for item in self.iter_mut() {
                if let Some(container) = item.container_mut() {
                    let index = container.items.iter().position(|i| i.is_ammo(ammo_type));
                    if let Some(index) = index {
                        return Some(container.items.remove(index));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{
        BACKPACK, CLOAK, HAT, LEATHER_ARM_GUARD, QUIVER, RAGS, WOODEN_ARROW,
    };
    use crate::game::{AmmoType, Item};

    use super::Wear;

    #[test]
    fn test_cant_wear_two_items_in_one_slot() {
        let wear = Wear::new([(Item::new(CLOAK), 0)]);
        assert!(!wear.can_add(&Item::new(RAGS), 0));
    }

    #[test]
    fn test_can_wear_two_items_in_different_slots() {
        let wear = Wear::new([(Item::new(CLOAK), 0)]);
        assert!(wear.can_add(&Item::new(HAT), 0));
    }

    #[test]
    fn test_can_wear_two_items_in_different_layers() {
        let wear = Wear::new([(Item::new(CLOAK), 0)]);
        assert!(wear.can_add(&Item::new(BACKPACK), 0));
    }

    #[test]
    fn test_can_wear_two_oversleeves_in_different_slots() {
        let mut wear = Wear::new([(Item::new(CLOAK), 0)]);
        let oversleeve = Item::new(LEATHER_ARM_GUARD);
        assert!(wear.can_add(&oversleeve, 0));
        wear.add(oversleeve.clone(), 0);
        assert!(wear.can_add(&oversleeve, 1));
        wear.add(oversleeve.clone(), 1);
        assert!(!wear.can_add(&oversleeve, 0));
        assert!(!wear.can_add(&oversleeve, 1));
    }

    #[test]
    fn test_has_ammo() {
        let wear = Wear::new([(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        )]);
        assert!(wear.has_ammo(&[AmmoType::Arrow].into()));
        assert!(!wear.has_ammo(&[AmmoType::Bolt].into()));

        let wear = Wear::new([(Item::new(QUIVER), 0)]);
        assert!(!wear.has_ammo(&[AmmoType::Arrow].into()));
    }

    #[test]
    fn test_get_ammo() {
        let wear = Wear::new([(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        )]);
        assert!(wear.get_ammo(&[AmmoType::Arrow].into()).is_some());
        assert!(wear.get_ammo(&[AmmoType::Bolt].into()).is_none());

        let arrow = wear.get_ammo(&[AmmoType::Arrow].into()).unwrap();
        assert_eq!(arrow.proto().id, WOODEN_ARROW);
    }

    #[test]
    fn test_remove_ammo() {
        let mut wear = Wear::new([(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        )]);
        let arrow = wear.remove_ammo(&[AmmoType::Arrow].into());
        assert!(arrow.is_some());
        let arrow = arrow.unwrap();
        assert_eq!(arrow.proto().id, WOODEN_ARROW);

        assert!(wear.remove_ammo(&[AmmoType::Bolt].into()).is_none());
        assert!(wear.remove_ammo(&[AmmoType::Arrow].into()).is_none());
    }
}
