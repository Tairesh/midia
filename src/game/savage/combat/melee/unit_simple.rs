use crate::game::{Avatar, BodySlot};

use super::{
    super::{HitCauses, HitParams, HitResult},
    fighting_roll,
};

// TODO: special attacks: both hands, kick, agressive, etc.
// TODO: refactor this function to smaller ones
pub fn melee_attack_unit(attacker: &Avatar, defender: &Avatar) -> UnitAttackResult {
    // TODO: add +1 to hit for every ally
    // TODO: Attack of unarmed enemy while attacker is armed causes +2 to Fighting skill rolls
    // TODO: some traits make some avatars armed even if they don't have weapons
    let hit_roll = fighting_roll(attacker);
    let parry = defender.parry() as i8;

    if hit_roll >= parry {
        let delta = hit_roll - parry;
        let critical = delta >= 4;

        let melee_damage = attacker.melee_damage();
        let damage = melee_damage
            .damage
            .roll(&attacker.personality.char_sheet, critical, true);
        let penetration = melee_damage.penetration;

        let toughness = defender.personality.char_sheet.toughness() as i8;

        // TODO: attack random parts of the body
        let mut armor = defender.armor(BodySlot::Torso) as i8;
        armor -= penetration as i8;
        if armor < 0 {
            armor = 0;
        }
        let mut total_damage = damage as i8 - (toughness + armor);

        let mut shock = false;
        let mut wounds = 0;
        if total_damage >= 0 {
            // add wound if target already shocked
            if defender.personality.char_sheet.shock {
                wounds += 1;
            }
            shock = true;
            // add wound for every Ace
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
