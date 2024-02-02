use std::collections::VecDeque;

use crate::game::{AmmoType, BodySlot, Item};

use super::{Wear, Wield};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Inventory {
    // TODO: remove pub
    pub wield: Wield,
    wear: Wear,
}

impl Inventory {
    pub fn new(hands_count: usize) -> Self {
        Self {
            wield: Wield::new(hands_count),
            wear: Wear::new([]),
        }
    }

    pub fn humanoid() -> Self {
        Self::new(2)
    }

    pub fn monster() -> Self {
        Self::new(0)
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.wield.clear();
        self.wear.clear();
    }

    pub fn wield(&mut self, item: Item) {
        self.wield.wield(item);
    }

    pub fn can_wield_any(&self) -> Result<(), String> {
        self.wield.can_wield(false)
    }

    pub fn can_wield(&self, item: &Item) -> Result<(), String> {
        self.wield.can_wield(item.is_two_handed())
    }

    pub fn wear(&mut self, item: Item, variant: usize) {
        self.wear.add(item, variant);
    }

    pub fn can_wear(&self, item: &Item, variant: usize) -> bool {
        self.wear.can_add(item, variant)
    }

    pub fn main_hand(&self) -> Option<&Item> {
        self.wield.main_hand()
    }

    pub fn main_hand_mut(&mut self) -> Option<&mut Item> {
        self.wield.main_hand_mut()
    }

    pub fn main_hand_take(&mut self) -> Option<Item> {
        self.wield.main_hand_take()
    }

    pub fn has_ammo(&self, ammo_type: AmmoType) -> bool {
        self.wear.has_ammo(ammo_type)
    }

    pub fn iter_wear(&self) -> impl Iterator<Item = &Item> {
        self.wear.iter()
    }

    pub fn get_items_by_slot(&self, slot: BodySlot) -> Vec<&Item> {
        self.wear.get_items_by_slot(slot)
    }

    pub fn take_all(&mut self) -> Vec<Item> {
        let mut items = self.wield.take_all();
        items.extend(self.wear.take_all());
        items
    }

    pub fn get_ammo(&self, ammo_type: AmmoType) -> Option<&Item> {
        self.wear.get_ammo(ammo_type)
    }

    pub fn remove_by_proto(&mut self, proto_id: &str, ammo_type: AmmoType) -> Option<Item> {
        self.wear.remove_by_proto(proto_id, ammo_type)
    }

    pub fn swap_hands(&mut self) {
        self.wield.swap();
    }
}
