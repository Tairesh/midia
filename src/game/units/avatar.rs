#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::super::{
    map::items::helpers::{dead_body, CLOAK, HAT},
    races::Pronouns,
    savage::HitResult,
    units::Personality,
    Action, AttackType, BodySlot, DamageValue, GameData, Item, Wear, Wield,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub id: usize,
    pub personality: Personality,
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    // TODO: monsters with no hands
    pub wield: Wield,
    // TODO: custom struct with layers for dress and methods to return names and icons for UI
    pub wear: Wear,
    // TODO: stamina
    // TODO: traits
    selected_ammo: Option<String>,
}

impl Avatar {
    pub fn new(personality: Personality, pos: Point) -> Self {
        Avatar {
            id: 1,
            action: None,
            vision: TwoDimDirection::East,
            wield: Wield::new(personality.appearance.race.hands_count()),
            wear: Wear::default(),
            selected_ammo: None,
            personality,
            pos,
        }
    }

    // TODO: remove this and select dress in create character scene
    pub fn dressed_default(personality: Personality, pos: Point) -> Self {
        Self {
            wear: Wear::new([(Item::new(HAT), 0), (Item::new(CLOAK), 0)]),
            ..Self::new(personality, pos)
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

    pub fn pronounce(&self) -> Pronouns {
        if self.is_player() {
            Pronouns::YouYour
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
            items.append(&mut self.wield.take_all());
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
        self.attack_damage(AttackType::Melee).unwrap()
    }

    // TODO: move this somewhere else
    pub fn attack_damage(&self, attack_type: AttackType) -> Option<DamageValue> {
        match attack_type {
            AttackType::Melee => Some(
                self.wield
                    .main_hand()
                    .map_or(self.personality.appearance.race.natural_weapon().1, |w| {
                        w.melee_damage()
                    }),
            ),
            AttackType::Shoot => {
                // TODO: natural ranged weapons
                let weapon = self.wield.main_hand()?;
                if let Some(ammo) = self.selected_ammo() {
                    let proto = GameData::instance()
                        .items
                        .get(&ammo)
                        .unwrap_or_else(|| panic!("Undefined ammo: `{ammo}`"));
                    if let Some(ammo_value) = &proto.ammo {
                        let mut damage = weapon.ranged_damage().unwrap();
                        damage.damage.modifier += ammo_value.damage_modifier.damage;
                        damage.penetration += ammo_value.damage_modifier.penetration;
                        if let Some(dice) = ammo_value.damage_modifier.damage_dice {
                            damage.damage.dices.push(dice);
                        }

                        return Some(damage);
                    }
                }

                weapon.ranged_damage()
            }
            AttackType::Throw => self
                .wield
                .main_hand()
                .and_then(|weapon| weapon.damage(attack_type)),
        }
    }

    // TODO: move this to Wear, probably
    pub fn selected_ammo(&self) -> Option<String> {
        let weapon = self.wield.main_hand()?;
        if let Some(selected_ammo) = &self.selected_ammo {
            let proto = GameData::instance()
                .items
                .get(selected_ammo.as_str())
                .unwrap();
            if let Some(ammo_value) = &proto.ammo {
                if ammo_value
                    .typ
                    .iter()
                    .any(|t| weapon.ammo_types().contains(t))
                {
                    return Some(selected_ammo.clone());
                }
            }
        }

        self.wear
            .get_ammo(weapon.ammo_types())
            .map(|a| a.proto().id.clone())
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::map::items::helpers::{
        CLOAK, GOD_AXE, QUIVER, STONE_ARROW, STONE_PIKE, WOODEN_ARROW, WOODEN_SHORTBOW,
    };
    use crate::game::traits::Name;
    use crate::game::units::tests::helpers::tester_girl;
    use crate::game::{AmmoType, AttackType, GameData};

    use super::{Avatar, BodySlot, HitResult, Item};

    #[test]
    fn test_npc_name() {
        let npc = Avatar::new(tester_girl(), Point::new(0, 0));

        assert_eq!(npc.name_for_actions(), "Dooka");
    }

    #[test]
    fn test_player_name() {
        let mut player = Avatar::new(tester_girl(), Point::new(0, 0));
        player.id = 0;

        assert_eq!(player.name_for_actions(), "you");
    }

    #[test]
    fn test_armor() {
        let mut avatar = Avatar::new(tester_girl(), Point::new(0, 0));
        avatar.wear.add(Item::new(CLOAK), 0);

        assert_eq!(avatar.armor(BodySlot::Torso), 1);
    }

    #[test]
    fn test_die() {
        let mut avatar = Avatar::new(tester_girl(), Point::new(0, 0));
        avatar.wield.wield(Item::new(GOD_AXE));
        avatar.wear.add(Item::new(CLOAK), 0);
        let items = avatar.apply_hit(HitResult::ultra_damage(), 0);
        assert_eq!(items.len(), 3);
        assert!(items.iter().any(|i| i.name() == "cloak"));
        assert!(items.iter().any(|i| i.name() == "god axe"));
        assert_eq!(items[2].name(), "dead gazan girl");
    }

    #[test]
    fn test_melee_damage() {
        let mut avatar = Avatar::new(tester_girl(), Point::new(0, 0));
        avatar.wield.wield(Item::new(GOD_AXE));

        let damage = avatar.melee_damage();
        let proto = GameData::instance()
            .items
            .get(GOD_AXE)
            .unwrap()
            .melee_damage
            .clone()
            .unwrap();

        assert_eq!(damage.damage.dices, proto.damage.dices);
        assert_eq!(damage.penetration, proto.penetration);
        assert_eq!(damage.attack_modifier, proto.attack_modifier);
        assert_eq!(damage.parry_modifier, proto.parry_modifier);
    }

    #[test]
    fn test_ranged_damage() {
        let mut avatar = Avatar::new(tester_girl(), Point::new(0, 0));
        avatar.wield.wield(Item::new(WOODEN_SHORTBOW));
        avatar.wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );

        let damage = avatar.attack_damage(AttackType::Shoot).unwrap();
        let proto = GameData::instance()
            .items
            .get(WOODEN_SHORTBOW)
            .unwrap()
            .ranged_damage
            .clone()
            .unwrap();
        let arrow_proto = GameData::instance()
            .items
            .get(WOODEN_ARROW)
            .unwrap()
            .ammo
            .clone()
            .unwrap();

        let mut dices = proto.damage.dices;
        if let Some(dice) = arrow_proto.damage_modifier.damage_dice {
            dices.push(dice);
        }
        assert_eq!(damage.damage.dices, dices);
        assert_eq!(
            damage.penetration,
            proto.penetration + arrow_proto.damage_modifier.penetration
        );
        assert_eq!(
            damage.damage.modifier,
            proto.damage.modifier + arrow_proto.damage_modifier.damage
        );
    }

    #[test]
    fn test_keep_selected_ammo() {
        let mut avatar = Avatar::new(tester_girl(), Point::new(0, 0));
        avatar.wield.wield(Item::new(WOODEN_SHORTBOW));
        avatar.wear.add(
            Item::new(QUIVER).with_items_inside([
                Item::new(WOODEN_ARROW),
                Item::new(WOODEN_ARROW),
                Item::new(STONE_ARROW),
            ]),
            0,
        );

        let selected_ammo = avatar.selected_ammo().unwrap();
        assert_eq!(selected_ammo, WOODEN_ARROW);

        let arrow = avatar.wear.remove_by_proto(&selected_ammo);
        assert!(arrow.is_some());
        let arrow = arrow.unwrap();
        assert_eq!(arrow.proto().id, WOODEN_ARROW);

        let selected_ammo = avatar.selected_ammo().unwrap();
        assert_eq!(selected_ammo, WOODEN_ARROW);

        let arrow = avatar.wear.remove_by_proto(&selected_ammo);
        assert!(arrow.is_some());
        let arrow = arrow.unwrap();
        assert_eq!(arrow.proto().id, WOODEN_ARROW);

        let selected_ammo = avatar.selected_ammo().unwrap();
        assert_eq!(selected_ammo, STONE_ARROW);

        let arrow = avatar.wear.remove_by_proto(&selected_ammo);
        assert!(arrow.is_some());
        let arrow = arrow.unwrap();
        assert_eq!(arrow.proto().id, STONE_ARROW);

        assert!(avatar.selected_ammo().is_none());
    }

    #[test]
    fn test_parry_modifier() {
        let mut avatar = Avatar::new(tester_girl(), Point::new(0, 0));
        let parry = avatar.parry();
        avatar.wield.wield(Item::new(STONE_PIKE));
        assert_eq!(avatar.parry(), parry - 1);
    }
}
