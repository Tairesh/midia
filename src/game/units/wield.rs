use std::collections::VecDeque;

use crate::game::traits::Name;

use super::super::{Item, ItemQuality};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Wield {
    items: VecDeque<Item>,
    hands_count: usize,
}

impl Wield {
    pub fn new(hands_count: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(hands_count),
            hands_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn main_hand(&self) -> Option<&Item> {
        self.items.front()
    }

    pub fn main_hand_mut(&mut self) -> Option<&mut Item> {
        self.items.front_mut()
    }

    pub fn off_hands(&self) -> Vec<&Item> {
        self.items.iter().skip(1).collect()
    }

    pub fn has_quality(&self, quality: &ItemQuality) -> bool {
        self.items.iter().any(|i| i.qualities().contains(quality))
    }

    pub fn take_from_active_hand(&mut self) -> Option<Item> {
        self.items.pop_front()
    }

    pub fn take_all(&mut self) -> Vec<Item> {
        self.items.drain(..).collect()
    }

    pub fn can_wield(&self, two_handed: bool) -> Result<(), String> {
        let free_hands = self.hands_count
            - self
                .items
                .iter()
                .map(|i| if i.is_two_handed() { 2 } else { 1 })
                .sum::<usize>();
        if free_hands == 0 {
            Err("No free hands".to_string())
        } else if two_handed && free_hands == 1 {
            Err("No free hands for two-handed weapon".to_string())
        } else {
            Ok(())
        }
    }

    pub fn wield(&mut self, item: Item) {
        self.items.push_front(item);
    }

    pub fn swap(&mut self) {
        if let Some(item) = self.items.pop_front() {
            self.items.push_back(item);
        }
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.items.clear();
    }
}
