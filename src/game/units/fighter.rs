use geometry::Point;

use super::{
    super::{
        map::items::helpers::dead_body, savage::HitResult, AttackType, BodySlot, CharSheet, Item,
    },
    Avatar, Weapon,
};

pub trait Fighter {
    fn as_avatar(&self) -> &dyn Avatar;
    fn id(&self) -> usize {
        self.as_avatar().id()
    }
    fn pos(&self) -> Point {
        self.as_avatar().pos()
    }
    fn armor(&self, slot: BodySlot) -> i8;
    fn weapon(&self, _attack_type: AttackType) -> Option<Weapon>;
    fn toughness(&self) -> u8 {
        self.as_avatar().char_sheet().toughness()
    }
    fn parry(&self) -> u8 {
        (self.as_avatar().char_sheet().parry() as i8
            + self
                .weapon(AttackType::Melee)
                .map_or(0, |w| w.damage.parry_modifier))
        .max(0) as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{
        CLOAK, GOD_AXE, QUIVER, STONE_PIKE, WOODEN_ARROW, WOODEN_SHORTBOW,
    };
    use crate::game::GameData;

    use super::{
        super::{tests::helpers::tester_girl, *},
        *,
    };

    #[test]
    fn test_armor() {
        let mut avatar = Player::new(tester_girl(), Point::new(0, 0));
        avatar.inventory_mut().unwrap().wear(Item::new(CLOAK), 0);

        assert_eq!(avatar.armor(BodySlot::Torso), 1);
    }

    #[test]
    fn test_melee_damage() {
        let mut avatar = Player::new(tester_girl(), Point::new(0, 0));
        avatar.inventory_mut().unwrap().wield(Item::new(GOD_AXE));

        let damage = avatar.weapon(AttackType::Melee).unwrap().damage;
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
        let mut avatar = Player::new(tester_girl(), Point::new(0, 0));
        avatar
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW));
        avatar.inventory_mut().unwrap().wear(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );
        avatar.inventory.reload().ok();

        let damage = avatar.weapon(AttackType::Shoot).unwrap().damage;
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
            .is_ammo
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
    fn test_parry_modifier() {
        let mut avatar = Player::new(tester_girl(), Point::new(0, 0));
        let parry = avatar.parry();
        avatar.inventory_mut().unwrap().wield(Item::new(STONE_PIKE));
        assert_eq!(avatar.parry(), parry - 1);
    }
}
