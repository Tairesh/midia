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

    pub fn second_hand(&self) -> Option<&Item> {
        self.wield.second_hand()
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

    pub fn swap_hands(&mut self) {
        self.wield.swap();
    }

    pub fn reload(&mut self) -> Result<(), ()> {
        let weapon = self.main_hand().ok_or(())?;
        let need_ammo = weapon.need_ammo().ok_or(())?;

        if need_ammo.capacity == 0 {
            return Err(());
        }

        for _ in 0..need_ammo.capacity {
            let new_ammo = self.wear.remove_ammo(need_ammo.typ);
            if let Some(new_ammo) = new_ammo {
                self.main_hand_mut()
                    .unwrap()
                    .container_mut()
                    .unwrap()
                    .push_item(new_ammo);
            }
        }

        Ok(())
    }
}
