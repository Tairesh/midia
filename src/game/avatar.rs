#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use crate::game::map::items::helpers::{cloak, hat};
use crate::game::BodySlot;

use super::{races::Personality, savage::CharSheet, Action, Item};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub personality: Personality,
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    // TODO: custom struct with hands counter and methods to return names and icons for UI
    pub wield: Vec<Item>,
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
            wield: Vec::new(),
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
