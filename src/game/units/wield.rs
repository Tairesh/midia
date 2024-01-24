use crate::game::traits::Name;

use super::super::{Item, ItemQuality, MainHand};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Wield {
    items: [Option<Item>; 2],
    active_hand: bool,
}

impl Wield {
    pub fn new(active_hand: bool) -> Self {
        Self {
            items: [None, None],
            active_hand,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.iter().all(Option::is_none)
    }

    pub fn swap_items(&mut self) {
        self.items.swap(0, 1);
    }

    pub fn left_hand(&self) -> Option<&Item> {
        self.get_item(0)
    }

    pub fn right_hand(&self) -> Option<&Item> {
        self.get_item(1)
    }

    pub fn main_hand(&self, main_hand: MainHand) -> Option<&Item> {
        match main_hand {
            MainHand::Left => self.left_hand(),
            _ => self.right_hand(),
        }
    }

    pub fn second_hand(&self, main_hand: MainHand) -> Option<&Item> {
        match main_hand {
            MainHand::Left => self.right_hand(),
            _ => self.left_hand(),
        }
    }

    pub fn has_quality(&self, quality: &ItemQuality) -> bool {
        self.items.iter().any(|i| {
            if let Some(i) = i {
                i.qualities().contains(quality)
            } else {
                false
            }
        })
    }

    pub fn active_hand(&self) -> Option<&Item> {
        self.get_item(self.active_hand_index())
    }

    pub fn take_from_active_hand(&mut self) -> Option<Item> {
        let i = self.active_hand_index();
        if self.items[i].is_some() {
            self.items[i].take()
        } else if self.items[1 - i]
            .as_ref()
            .filter(|i| i.is_two_handed())
            .is_some()
        {
            self.items[1 - i].take()
        } else {
            None
        }
    }

    pub fn off_hand(&self) -> Option<&Item> {
        self.get_item(1 - self.active_hand_index())
    }

    pub fn take_from_off_hand(&mut self) -> Option<Item> {
        let i = self.active_hand_index();
        if self.items[1 - i].is_some() {
            self.items[1 - i].take()
        } else if self.items[i]
            .as_ref()
            .filter(|i| i.is_two_handed())
            .is_some()
        {
            self.items[i].take()
        } else {
            None
        }
    }

    pub fn can_wield(&self, two_handed: bool) -> Result<(), String> {
        if let Some(item) = self.active_hand() {
            return Err(format!("You already have {} in main hand", item.name(),));
        } else if let Some(item) = self.off_hand() {
            if item.is_two_handed() || two_handed {
                return Err(format!("You already have {} in off hand", item.name()));
            }
        }

        Ok(())
    }

    pub fn wield(&mut self, item: Item) {
        self.items[self.active_hand_index()] = Some(item);
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.items.iter_mut().for_each(|i| *i = None);
    }

    fn active_hand_index(&self) -> usize {
        self.active_hand.into()
    }

    fn get_item(&self, index: usize) -> Option<&Item> {
        if let Some(item) = self.items[index].as_ref() {
            Some(item)
        } else {
            self.items[1 - index].as_ref().filter(|i| i.is_two_handed())
        }
    }
}
