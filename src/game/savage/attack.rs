use crate::game::{Avatar, BodySlot, Dice, DiceWithModifier, MeleeDamageValue, Skill};

use super::Wound;

pub enum AttackResult {
    Miss,
    Hit(HitResult),
}

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

pub struct HitCauses {
    pub shock: bool,
    pub wounds: Vec<Wound>,
}

impl HitCauses {
    pub fn random_wounds(shock: bool, wounds: u8) -> Self {
        Self {
            shock,
            wounds: (0..wounds).map(|_| Wound::random()).collect(),
        }
    }
}

pub struct HitResult {
    pub params: HitParams,
    pub causes: HitCauses,
}

impl HitResult {
    pub fn new(params: HitParams, causes: HitCauses) -> Self {
        Self { params, causes }
    }
}

// TODO: attack terrains and items
// TODO: refactor this function to smaller ones
pub fn melee_attack(attacker: &Avatar, defender: &Avatar) -> AttackResult {
    let weapon = attacker.wield.active_hand();
    // TODO: add +1 to hit for every ally
    let hit_dice = attacker
        .char_sheet
        .get_skill_with_modifiers(Skill::Fighting);
    let hit_roll = if attacker.char_sheet.wild_card {
        let wild_dice = DiceWithModifier::new(Dice::D6, hit_dice.modifier());
        u8::max(hit_dice.roll(), wild_dice.roll_explosive())
    } else {
        hit_dice.roll()
    };
    // TODO: Attack of unarmed enemy while attacker is armed causes +2 to Fighting skill rolls

    let parry = defender.char_sheet.parry();
    if hit_roll >= parry {
        let delta = hit_roll - parry;
        let critical = delta >= 4;

        let damage_params = if let Some(weapon) = weapon {
            weapon.melee_damage()
        } else {
            MeleeDamageValue::default()
        };
        let damage = damage_params.damage.roll(&attacker.char_sheet, critical);
        let penetration = damage_params.penetration;

        let toughness = defender.char_sheet.toughness();
        // TODO: attack random parts of the body
        let mut armor = defender.armor(BodySlot::Torso);
        armor = armor.saturating_sub(penetration);
        let mut total_damage = damage.saturating_sub(toughness + armor);

        let mut shock = false;
        let mut wounds = 0;
        if total_damage > 0 {
            // add wound if target already shocked
            if defender.char_sheet.shock {
                wounds += 1;
            }
            shock = true;
            // add wound for every success
            while total_damage > 4 {
                wounds += 1;
                total_damage -= 4;
            }
        }

        AttackResult::Hit(HitResult::new(
            HitParams::new(damage, penetration, critical),
            HitCauses::random_wounds(shock, wounds),
        ))
    } else {
        AttackResult::Miss
    }
}

// TODO: special attacks: both hands, kick, agressive, etc.
// TODO: distance attacks: bows, throwing, and melee weapons with distance > 0
