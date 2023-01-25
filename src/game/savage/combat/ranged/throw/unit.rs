use geometry::DIR8;

use crate::game::savage::HitResult;
use crate::game::{Avatar, RollResult, Skill, World};

use super::super::{RangedDistance, UnitRangedAttackResult};

// TODO: grenades
pub fn throw_attack_unit(
    attacker: &Avatar,
    defender: &Avatar,
    world: &World,
) -> UnitRangedAttackResult {
    let throw_roll = throwing_roll(attacker, defender);
    if throw_roll.is_none() {
        return UnitRangedAttackResult::Impossible;
    }
    let throw_roll = throw_roll.unwrap();

    if throw_roll.natural == 1 {
        let units_to_hit = DIR8
            .into_iter()
            .flat_map(|dir| world.map().get_tile(defender.pos + dir).units.clone())
            .collect::<Vec<_>>();
        if units_to_hit.is_empty() {
            return UnitRangedAttackResult::Miss;
        }
        let random_target = units_to_hit[rand::random::<usize>() % units_to_hit.len()];
        let target = world.get_unit(random_target);

        UnitRangedAttackResult::InnocentBystander(
            random_target,
            calculate_hit(attacker, target, false),
        )
    } else if throw_roll.total < 4 {
        UnitRangedAttackResult::Miss
    } else {
        UnitRangedAttackResult::Hit(calculate_hit(
            attacker,
            defender,
            (throw_roll.total - 4) >= 4,
        ))
    }
}

fn throwing_roll(attacker: &Avatar, defender: &Avatar) -> Option<RollResult> {
    let distance = attacker.pos.distance(defender.pos);
    let damage_value = attacker.throw_damage().unwrap();
    let distance = RangedDistance::define(distance, damage_value.distance);

    if distance == RangedDistance::Unreachable {
        return None;
    }

    Some(attacker.personality.char_sheet.roll_skill(
        Skill::Athletics,
        attacker.melee_damage().attack_modifier + distance.modifier(),
    ))
}

fn calculate_hit(attacker: &Avatar, defender: &Avatar, critical: bool) -> HitResult {
    let damage_value = attacker.throw_damage().unwrap();
    let damage = damage_value.roll(&attacker.personality.char_sheet, critical, true);

    HitResult::calculate(damage.damage, damage.penetration, defender, critical)
}
