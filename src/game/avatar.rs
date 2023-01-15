#![allow(dead_code)]

use std::collections::VecDeque;

use geometry::{Point, TwoDimDirection};

use super::{
    map::items::helpers::{cloak, hat},
    races::Personality,
    savage::CharSheet,
    Action, BodySlot, Item, ItemQuality,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Wield {
    items: VecDeque<Item>,
}

impl Wield {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn has_free_space(&self) -> bool {
        self.is_empty() || (self.items.len() < 2 && !self.items[0].two_handed())
    }

    pub fn can_wield(&self, item: &Item) -> bool {
        if item.two_handed() {
            self.is_empty()
        } else {
            self.has_free_space()
        }
    }

    pub fn switch_sides(&mut self) -> bool {
        if self.items.len() == 2 {
            self.items.swap(0, 1);
            true
        } else {
            false
        }
    }

    pub fn names(&self) -> String {
        let names = self.items.iter().map(Item::name).collect::<Vec<&str>>();
        if names.is_empty() {
            "nothing".to_string()
        } else {
            names.join(" and ")
        }
    }

    pub fn has_quality(&self, quality: ItemQuality) -> bool {
        self.items.iter().any(|i| i.qualities().contains(&quality))
    }

    pub fn get_item(&self) -> Option<&Item> {
        self.items.get(0)
    }

    pub fn take_item(&mut self) -> Option<Item> {
        self.items.pop_front()
    }

    pub fn wield(&mut self, item: Item) {
        self.items.push_front(item);
    }

    #[cfg(test)]
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub personality: Personality,
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    // TODO: custom struct with hands counter and methods to return names and icons for UI
    pub wield: Wield,
    // TODO: custom struct with layers for dress and methods to return names and icons for UI
    pub wear: Vec<Item>,
    pub char_sheet: CharSheet,
    // TODO: stamina
    // TODO: traits
}

impl Avatar {
    pub fn new(personality: Personality, char_sheet: CharSheet, pos: Point) -> Self {
        Avatar {
            personality,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Wield::default(),
            wear: Vec::new(),
            char_sheet,
        }
    }

    // TODO: remove this and select dress in create character scene
    pub fn dressed_default(personality: Personality, char_sheet: CharSheet, pos: Point) -> Self {
        Self {
            wear: vec![hat(), cloak()],
            ..Self::new(personality, char_sheet, pos)
        }
    }

    pub fn name_for_actions(&self) -> String {
        if self.is_player() {
            "You".to_string()
        } else {
            self.personality.mind.name.clone()
        }
    }

    pub fn is_player(&self) -> bool {
        self.personality.is_player
    }

    pub fn armor(&self, slot: BodySlot) -> u8 {
        self.wear.iter().map(|item| item.armor(slot)).sum()
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::races::tests::personality::tester_girl;
    use crate::game::races::Race;
    use crate::game::BodySlot;

    use super::{Avatar, CharSheet};

    #[test]
    fn test_npc_name() {
        let npc = Avatar::new(
            tester_girl(),
            CharSheet::default(Race::Gazan, 15),
            Point::new(0, 0),
        );

        assert_eq!(npc.name_for_actions(), "Dooka");
    }

    #[test]
    fn test_player_name() {
        let mut personality = tester_girl();
        personality.is_player = true;
        let player = Avatar::new(
            personality,
            CharSheet::default(Race::Gazan, 15),
            Point::new(0, 0),
        );

        assert_eq!(player.name_for_actions(), "You");
    }

    #[test]
    fn test_armor() {
        let avatar = Avatar::dressed_default(
            tester_girl(),
            CharSheet::default(Race::Gazan, 15),
            Point::new(0, 0),
        );

        assert_eq!(avatar.armor(BodySlot::Torso), 1);
    }
}
