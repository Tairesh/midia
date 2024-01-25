use crate::game::{Attribute, Avatar, BodySlot, Wound};

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

    pub fn calculate(damage: u8, penetration: u8, target: &Avatar, critical: bool) -> Self {
        let toughness = target.personality.char_sheet.toughness() as i8;

        // TODO: attack random parts of the body
        // TODO: add damage_type and armor types
        let mut armor = target.armor(BodySlot::Torso) as i8;
        armor -= penetration as i8;
        if armor < 0 {
            armor = 0;
        }
        let mut total_damage = damage as i8 - (toughness + armor);

        let mut shock = false;
        let mut wounds = 0;
        if total_damage >= 0 {
            // add wound if target already shocked
            if target.personality.char_sheet.shock {
                wounds += 1;
            }
            shock = true;
            // add wound for every Ace
            while total_damage > 4 {
                wounds += 1;
                total_damage -= 4;
            }
        }

        // target can do a Vigor roll to avoid wounds
        let vigor_roll = target
            .personality
            .char_sheet
            .get_attribute_with_modifiers(Attribute::Vigor)
            .roll();
        if vigor_roll.successes() >= wounds {
            wounds = 0;
            shock = false;
        } else {
            wounds -= vigor_roll.successes();
        }

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
