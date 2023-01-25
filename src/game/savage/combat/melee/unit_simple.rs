use crate::game::Avatar;

use super::{super::HitResult, fighting_roll};

// TODO: special attacks: both hands, kick, agressive, etc.
pub fn melee_attack_unit(attacker: &Avatar, defender: &Avatar) -> UnitMeleeAttackResult {
    // TODO: add +1 to hit for every ally
    // TODO: Attack of unarmed enemy while attacker is armed causes +2 to Fighting skill rolls
    // TODO: some traits make some avatars armed even if they don't have weapons
    let hit_roll = fighting_roll(attacker);
    let parry = defender.parry() as i8;

    if hit_roll >= parry {
        let delta = hit_roll - parry;
        let critical = delta >= 4;

        let melee_damage = attacker.melee_damage();
        let damage = melee_damage.roll(&attacker.personality.char_sheet, critical, true);

        UnitMeleeAttackResult::Hit(HitResult::calculate(
            damage.damage,
            damage.penetration,
            defender,
            critical,
        ))
    } else {
        UnitMeleeAttackResult::Miss
    }
}

pub enum UnitMeleeAttackResult {
    Miss,
    Hit(HitResult),
}
