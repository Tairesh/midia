#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use crate::game::DamageValue;

use super::super::{
    map::items::helpers::{dead_body, CLOAK, HAT},
    races::{MainHand, Personality},
    savage::HitResult,
    Action, BodySlot, Item, Wear, Wield,
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
    pub wear: Wear,
    // TODO: stamina
    // TODO: traits
}

impl Avatar {
    pub fn new(id: usize, personality: Personality, pos: Point) -> Self {
        Avatar {
            id,
            action: None,
            vision: TwoDimDirection::East,
            wield: Wield::new(!matches!(personality.mind.main_hand, MainHand::Left)),
            wear: Wear::new(),
            personality,
            pos,
        }
    }

    // TODO: remove this and select dress in create character scene
    pub fn dressed_default(id: usize, personality: Personality, pos: Point) -> Self {
        let mut wear = Wear::new();
        wear.add(Item::new(HAT), 0);
        wear.add(Item::new(CLOAK), 0);
        Self {
            wear,
            ..Self::new(id, personality, pos)
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

    pub fn is_dead(&self) -> bool {
        self.personality.char_sheet.is_dead()
    }

    pub fn armor(&self, slot: BodySlot) -> u8 {
        self.wear
            .get_items_by_slot(slot)
            .into_iter()
            .map(Item::armor)
            .sum()
    }

    pub fn pronounce(&self) -> (&str, &str, &str) {
        if self.is_player() {
            ("you", "you", "your")
        } else {
            self.personality.mind.gender.pronounce()
        }
    }

    pub fn apply_hit(&mut self, hit: HitResult, current_tick: u128) -> Vec<Item> {
        self.personality.char_sheet.apply_hit(hit, current_tick);

        // TODO: drop weapons if arm is wounded

        if self.is_dead() {
            self.action = None;

            let mut items = Vec::new();
            items.append(&mut self.wear.take_all());
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

    pub fn parry(&self) -> u8 {
        (self.personality.char_sheet.parry() as i8 + self.melee_damage().parry_modifier).max(0)
            as u8
    }

    pub fn melee_damage(&self) -> DamageValue {
        if let Some(weapon) = self.wield.active_hand() {
            weapon.melee_damage()
        } else {
            self.personality.appearance.race.natural_weapon().1
        }
    }

    pub fn throw_damage(&self) -> Option<DamageValue> {
        if let Some(weapon) = self.wield.active_hand() {
            weapon.throw_damage()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::map::items::helpers::{CLOAK, GOD_AXE};
    use crate::game::races::tests::personality::tester_girl;

    use super::{Avatar, BodySlot, HitResult, Item};

    #[test]
    fn test_npc_name() {
        let npc = Avatar::new(1, tester_girl(), Point::new(0, 0));

        assert_eq!(npc.name_for_actions(), "Dooka");
    }

    #[test]
    fn test_player_name() {
        let player = Avatar::new(0, tester_girl(), Point::new(0, 0));

        assert_eq!(player.name_for_actions(), "you");
    }

    #[test]
    fn test_armor() {
        let mut avatar = Avatar::new(0, tester_girl(), Point::new(0, 0));
        avatar.wear.add(Item::new(CLOAK), 0);

        assert_eq!(avatar.armor(BodySlot::Torso), 1);
    }

    #[test]
    fn test_die() {
        let mut avatar = Avatar::new(0, tester_girl(), Point::new(0, 0));
        avatar.wield.wield(Item::new(GOD_AXE));
        avatar.wear.add(Item::new(CLOAK), 0);
        let items = avatar.apply_hit(HitResult::ultra_damage(), 0);
        assert_eq!(items.len(), 3);
        assert!(items.iter().any(|i| i.name() == "cloak"));
        assert!(items.iter().any(|i| i.name() == "god axe"));
        assert_eq!(items[2].name(), "dead gazan girl");
    }
}
