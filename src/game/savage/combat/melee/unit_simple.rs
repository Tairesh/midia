use crate::game::{Avatar, BodySlot, Item, Skill};

use super::super::{HitCauses, HitParams, HitResult};

// TODO: special attacks: both hands, kick, agressive, etc.
// TODO: refactor this function to smaller ones
pub fn melee_attack_unit(attacker: &Avatar, defender: &Avatar) -> UnitAttackResult {
    // TODO: Attack of unarmed enemy while attacker is armed causes +2 to Fighting skill rolls
    // TODO: add +1 to hit for every ally
    let hit_roll = attacker.personality.char_sheet.roll_skill(Skill::Fighting);

    let parry = defender.personality.char_sheet.parry();
    if hit_roll >= parry {
        let delta = hit_roll - parry;
        let critical = delta >= 4;

        let damage_params = attacker
            .wield
            .active_hand()
            .map(Item::melee_damage)
            .unwrap_or_default();
        let damage = damage_params
            .damage
            .roll(&attacker.personality.char_sheet, critical, true);
        let penetration = damage_params.penetration;

        let toughness = defender.personality.char_sheet.toughness();
        // TODO: attack random parts of the body
        let mut armor = defender.armor(BodySlot::Torso);
        armor = armor.saturating_sub(penetration);
        let mut total_damage = damage.saturating_sub(toughness + armor);

        let mut shock = false;
        let mut wounds = 0;
        if total_damage > 0 {
            // add wound if target already shocked
            if defender.personality.char_sheet.shock {
                wounds += 1;
            }
            shock = true;
            // add wound for every success
            while total_damage > 4 {
                wounds += 1;
                total_damage -= 4;
            }
        }

        UnitAttackResult::Hit(HitResult::new(
            HitParams::new(damage, penetration, critical),
            HitCauses::random_wounds(shock, wounds),
        ))
    } else {
        UnitAttackResult::Miss
    }
}

pub enum UnitAttackResult {
    Miss,
    Hit(HitResult),
}
