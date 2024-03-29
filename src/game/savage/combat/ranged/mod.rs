use std::collections::HashMap;

use geometry::{Point, DIR8};

pub use distance::RangedDistance;

use crate::game::{AttackType, Avatar, Fighter, RollResult, Skill, World};

use super::HitResult;

mod distance;

#[derive(Debug)]
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
    attacker: &dyn Fighter,
    defender: &dyn Fighter,
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
            .flat_map(|dir| world.map().get_tile(defender.pos() + dir).units.clone())
            .collect::<Vec<_>>();
        if units_to_hit.is_empty() {
            return UnitRangedAttackResult::Miss;
        }
        let random_target = units_to_hit[rand::random::<usize>() % units_to_hit.len()];

        UnitRangedAttackResult::InnocentBystander(
            random_target,
            calculate_hit(
                attack_type,
                attacker,
                world.units().get_unit(random_target).as_fighter(),
                false,
            ),
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
    attacker: &dyn Fighter,
    defender: &dyn Fighter,
) -> Option<RollResult> {
    let distance = attacker.pos().distance_to(defender.pos());
    let damage_value = attacker.weapon(attack_type).unwrap().damage;
    let distance = RangedDistance::define(distance, damage_value.distance);

    if distance == RangedDistance::Unreachable {
        return None;
    }

    Some(attacker.as_avatar().char_sheet().roll_skill(
        match attack_type {
            AttackType::Throw => Skill::Athletics,
            AttackType::Shoot => Skill::Shooting,
            AttackType::Melee => Skill::Fighting,
        },
        damage_value.attack_modifier + distance.modifier(),
    ))
}

fn calculate_hit(
    attack_type: AttackType,
    attacker: &dyn Fighter,
    defender: &dyn Fighter,
    critical: bool,
) -> HitResult {
    let damage_value = attacker.weapon(attack_type).unwrap().damage;
    let damage = damage_value.roll(attacker.as_avatar().char_sheet(), critical, true);

    HitResult::calculate(damage.damage, damage.penetration, defender, critical)
}
