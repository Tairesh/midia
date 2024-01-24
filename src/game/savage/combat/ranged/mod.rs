use std::collections::HashMap;

use geometry::{Point, DIR8};

pub use distance::RangedDistance;

use crate::game::{AttackType, Avatar, RollResult, Skill, World};

use super::HitResult;

mod distance;

pub enum UnitRangedAttackResult {
    InnocentBystander(usize, HitResult),
    Miss,
    Hit(HitResult),
    #[allow(dead_code)] // TODO: explosions
    Explosion(HashMap<usize, HitResult>, HashMap<Point, u8>),
    Impossible,
}

pub fn ranged_attack_unit(
    attack_type: AttackType,
    attacker: &Avatar,
    defender: &Avatar,
    world: &World,
) -> UnitRangedAttackResult {
    let throw_roll = attack_roll(attack_type, attacker, defender);
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
        let target = world.units.get_unit(random_target);

        UnitRangedAttackResult::InnocentBystander(
            random_target,
            calculate_hit(attack_type, attacker, target, false),
        )
    } else if throw_roll.total < 4 {
        UnitRangedAttackResult::Miss
    } else {
        UnitRangedAttackResult::Hit(calculate_hit(
            attack_type,
            attacker,
            defender,
            (throw_roll.total - 4) >= 4,
        ))
    }
}

fn attack_roll(
    attack_type: AttackType,
    attacker: &Avatar,
    defender: &Avatar,
) -> Option<RollResult> {
    let distance = attacker.pos.distance(defender.pos);
    let damage_value = attacker.attack_damage(attack_type).unwrap();
    let distance = RangedDistance::define(distance, damage_value.distance);

    if distance == RangedDistance::Unreachable {
        return None;
    }

    Some(attacker.personality.char_sheet.roll_skill(
        match attack_type {
            AttackType::Throw => Skill::Athletics,
            AttackType::Shoot => Skill::Shooting,
            AttackType::Melee => Skill::Fighting,
        },
        attacker.attack_damage(attack_type).unwrap().attack_modifier + distance.modifier(),
    ))
}

fn calculate_hit(
    attack_type: AttackType,
    attacker: &Avatar,
    defender: &Avatar,
    critical: bool,
) -> HitResult {
    let damage_value = attacker.attack_damage(attack_type).unwrap();
    let damage = damage_value.roll(&attacker.personality.char_sheet, critical, true);

    HitResult::calculate(damage.damage, damage.penetration, defender, critical)
}
