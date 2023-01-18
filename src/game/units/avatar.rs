#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::super::{
    map::items::helpers::{cloak, dead_body, hat},
    races::{MainHand, Personality},
    savage::CharSheet,
    Action, BodySlot, HitResult, Item, Wield,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub id: usize,
    pub personality: Personality,
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    pub wield: Wield,
    // TODO: custom struct with layers for dress and methods to return names and icons for UI
    pub wear: Vec<Item>,
    pub char_sheet: CharSheet,
    // TODO: stamina
    // TODO: traits
}

impl Avatar {
    pub fn new(id: usize, personality: Personality, char_sheet: CharSheet, pos: Point) -> Self {
        Avatar {
            id,
            action: None,
            vision: TwoDimDirection::East,
            wield: Wield::new(!matches!(personality.mind.main_hand, MainHand::Left)),
            wear: Vec::new(),
            personality,
            char_sheet,
            pos,
        }
    }

    // TODO: remove this and select dress in create character scene
    pub fn dressed_default(
        id: usize,
        personality: Personality,
        char_sheet: CharSheet,
        pos: Point,
    ) -> Self {
        Self {
            wear: vec![hat(), cloak()],
            ..Self::new(id, personality, char_sheet, pos)
        }
    }

    pub fn name_for_actions(&self) -> String {
        if self.is_player() {
            "you".to_string()
        } else {
            self.personality.mind.name.clone()
        }
    }

    pub fn is_player(&self) -> bool {
        self.id == 0
    }

    pub fn armor(&self, slot: BodySlot) -> u8 {
        self.wear.iter().map(|item| item.armor(slot)).sum()
    }

    pub fn pronounce(&self) -> (&str, &str, &str) {
        if self.is_player() {
            ("you", "you", "your")
        } else {
            self.personality.mind.gender.pronounce()
        }
    }

    pub fn apply_hit(&mut self, hit: HitResult, current_tick: u128) -> Vec<Item> {
        self.char_sheet.apply_hit(hit, current_tick);

        // TODO: drop weapons if arm is wounded

        if self.char_sheet.is_dead() {
            self.action = None;

            let mut items = Vec::new();
            items.append(&mut self.wear);
            if let Some(item) = self.wield.take_from_active_hand() {
                items.push(item);
            }
            if let Some(item) = self.wield.take_from_off_hand() {
                items.push(item);
            }
            items.push(dead_body(self));
            items
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::map::items::helpers::axe;
    use crate::game::races::tests::personality::tester_girl;
    use crate::game::races::Race;
    use crate::game::{BodySlot, HitResult};

    use super::{Avatar, CharSheet};

    #[test]
    fn test_npc_name() {
        let npc = Avatar::new(
            1,
            tester_girl(),
            CharSheet::default(true, Race::Gazan, 15),
            Point::new(0, 0),
        );

        assert_eq!(npc.name_for_actions(), "Dooka");
    }

    #[test]
    fn test_player_name() {
        let player = Avatar::new(
            0,
            tester_girl(),
            CharSheet::default(true, Race::Gazan, 15),
            Point::new(0, 0),
        );

        assert_eq!(player.name_for_actions(), "you");
    }

    #[test]
    fn test_armor() {
        let avatar = Avatar::dressed_default(
            0,
            tester_girl(),
            CharSheet::default(true, Race::Gazan, 15),
            Point::new(0, 0),
        );

        assert_eq!(avatar.armor(BodySlot::Torso), 1);
    }

    #[test]
    fn test_die() {
        let mut avatar = Avatar::dressed_default(
            0,
            tester_girl(),
            CharSheet::default(false, Race::Gazan, 15),
            Point::new(0, 0),
        );
        avatar.wield.wield(axe());
        let items = avatar.apply_hit(HitResult::ultra_damage(), 0);
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].name(), "strange hat");
        assert_eq!(items[1].name(), "cloak");
        assert_eq!(items[2].name(), "axe");
        assert_eq!(items[3].name(), "dead gazan girl");
    }
}
