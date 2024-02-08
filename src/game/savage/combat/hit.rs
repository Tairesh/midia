use crate::game::{Attribute, Avatar, BodySlot, Fighter, Wound};

#[derive(Debug)]
pub struct HitResult {
    pub params: HitParams,
    pub consequences: HitConsequences,
}

impl HitResult {
    pub fn new(params: HitParams, consequences: HitConsequences) -> Self {
        Self {
            params,
            consequences,
        }
    }

    pub fn calculate(damage: u8, penetration: u8, target: &dyn Fighter, critical: bool) -> Self {
        let toughness = target.toughness() as i8;

        // TODO: attack random parts of the body
        // TODO: add damage_type and armor types
        let mut armor = target.armor(BodySlot::Torso);
        armor -= penetration as i8;
        if armor < 0 {
            armor = 0;
        }
        let mut total_damage = damage as i8 - (toughness + armor);

        let mut shock = false;
        let mut wounds = 0;
        if total_damage >= 0 {
            // add wound if target already shocked
            if target.as_avatar().char_sheet().shock {
                wounds += 1;
            }
            shock = true;
            // add wound for every Ace
            while total_damage >= 4 {
                wounds += 1;
                total_damage -= 4;
            }
        }

        // TODO: wild cards can do a Vigor roll to avoid wounds

        Self::new(
            HitParams::new(damage, penetration, critical),
            HitConsequences::random_wounds(shock, wounds),
        )
    }

    #[cfg(test)]
    pub fn ultra_damage() -> Self {
        Self::new(
            HitParams::new(100, 100, true),
            HitConsequences::random_wounds(true, 4),
        )
    }
}

#[derive(Debug)]
pub struct HitParams {
    pub damage: u8,
    pub penetration: u8,
    pub critical: bool,
}

impl HitParams {
    pub fn new(damage: u8, penetration: u8, critical: bool) -> Self {
        Self {
            damage,
            penetration,
            critical,
        }
    }
}

#[derive(Debug)]
pub struct HitConsequences {
    pub shock: bool,
    pub wounds: Vec<Wound>,
}

impl HitConsequences {
    pub fn nothing() -> Self {
        Self {
            shock: false,
            wounds: Vec::new(),
        }
    }

    pub fn random_wounds(shock: bool, wounds: u8) -> Self {
        Self {
            shock,
            wounds: (0..wounds).map(|_| Wound::random()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;
    use test_case::test_case;

    use crate::game::units::tests::helpers::shasha;
    use crate::game::units::{Fighter, Player};

    use super::*;

    #[test_case(4, 0, false, 0)]
    #[test_case(5, 0, false, 0)]
    #[test_case(7, 0, true, 0)]
    #[test_case(5, 2, true, 0)]
    #[test_case(5, 10, true, 0)]
    #[test_case(8, 2, true, 0)]
    #[test_case(9, 2, true, 1)]
    #[test_case(10, 0, true, 0)]
    #[test_case(11, 0, true, 1)]
    fn test_hit_result(damage: u8, penetration: u8, shock: bool, wounds: usize) {
        let avatar = Player::new(shasha(), Point::default());
        let parry = avatar.parry();
        assert_eq!(parry, 8);
        let toughness = avatar.toughness();
        assert_eq!(toughness, 5);
        let armor = avatar.armor(BodySlot::Torso);
        assert_eq!(armor, 2);

        let hit_result = HitResult::calculate(damage, penetration, &avatar, false);
        assert_eq!(hit_result.consequences.shock, shock);
        assert_eq!(hit_result.consequences.wounds.len(), wounds);
    }
}
