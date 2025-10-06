/// The `ranged_attack` module provides functions for shoot and throw actions.
use super::super::{
    Action,
    ActionPossibility::{self, No, Yes},
    AttackTarget,
};
use crate::game::{
    game_data::NeedAmmoValue,
    log::helpers::unit_attack_success,
    savage::{ranged_attack_unit, UnitRangedAttackResult, ATTACK_MOVES},
    traits::Name,
    units::Inventory,
    AttackType, Avatar, LogEvent, RangedDistance, World,
};
use crate::lang::a;
use roguemetry::Point;
use std::fmt::format;

/// Checks if a ranged attack is possible.
///
/// # Arguments
///
/// * `actor_id` - The ID of the actor performing the attack.
/// * `attack_target` - The target of the attack.
/// * `world` - The current game world.
/// * `attack_type` - The type of the attack (throw or shoot).
///
/// # Returns
///
/// * `ActionPossibility` - An enum indicating whether the attack is possible, and if not, why not.
pub fn is_possible(
    actor_id: usize,
    attack_target: AttackTarget,
    world: &World,
    attack_type: AttackType,
) -> ActionPossibility {
    let actor = world.units.get_unit(actor_id);
    if actor.char_sheet().shock {
        return No("You are in shock".to_string());
    }

    if actor.as_fighter().weapon(attack_type).is_none() {
        return No(format!(
            "You have nothing to {}.",
            attack_type.verb_a(false)
        ));
    }

    // TODO: check for natural weapon
    let weapon = actor.inventory().unwrap().main_hand().unwrap();

    if let Some(NeedAmmoValue { typ, .. }) = weapon.need_ammo() {
        if !weapon.has_ammo(typ) {
            return No(format!("You have no ammo in {}!", a(weapon.name())));
        }
    }

    let damage_value = match attack_type {
        AttackType::Throw => {
            if let Some(throw_value) = weapon.throw_damage() {
                throw_value
            } else {
                return No(format!("You can't throw {}.", a(weapon.name())));
            }
        }
        AttackType::Shoot => {
            if let Some(ranged_value) = weapon.ranged_damage() {
                ranged_value
            } else {
                return No(format!("You can't shoot from {}.", a(weapon.name())));
            }
        }
        AttackType::Melee => unreachable!(),
    };
    let close_distance = damage_value.distance;
    if close_distance == 0 {
        return No(format!(
            "You can't {} {}.",
            attack_type.verb_a(false),
            a(weapon.name())
        ));
    }

    let pos = attack_target.pos(world);
    let distance = RangedDistance::define(actor.pos().distance_to(pos), close_distance);

    match distance {
        RangedDistance::Unreachable => No(format!(
            "You can't {} {} that far.",
            attack_type.verb_a(false),
            a(weapon.name())
        )),
        RangedDistance::Melee => {
            if attack_type == AttackType::Shoot {
                No("You can't shoot in closed combat.".to_string())
            } else if attack_target.is_avatar() {
                No("You can't throw in closed combat.".to_string())
            } else {
                Yes(ATTACK_MOVES)
            }
        }
        _ => Yes(ATTACK_MOVES),
    }
}

/// Handles the result of a ranged attack.
///
/// # Arguments
///
/// * `action` - The action being performed.
/// * `attack_target` - The target of the attack.
/// * `world` - The current game world.
/// * `attack_type` - The type of the attack (throw or shoot).
pub fn on_finish(
    action: &Action,
    attack_target: AttackTarget,
    world: &mut World,
    attack_type: AttackType,
) {
    match attack_target {
        AttackTarget::Terrain(pos) => finish_terrain(pos, world, attack_type, action),
        AttackTarget::Avatar(unit_id) => finish_unit(unit_id, world, attack_type, action),
    }
}

/// Handles the result of a ranged attack on a unit.
///
/// # Arguments
///
/// * `unit_id` - The ID of the unit being attacked.
/// * `world` - The current game world.
/// * `attack_type` - The type of the attack (throw or shoot).
/// * `action` - The action being performed.
fn finish_unit(unit_id: usize, world: &mut World, attack_type: AttackType, action: &Action) {
    let attack_result = get_unit_attack_result(world, unit_id, attack_type, action);
    log_unit_attack_result(&attack_result, attack_type, world, unit_id, action.owner);

    let victim_id = match attack_result {
        UnitRangedAttackResult::InnocentBystander(victim_id, hit) => {
            world.apply_damage(victim_id, hit);
            victim_id
        }
        UnitRangedAttackResult::Hit(hit) => {
            world.apply_damage(unit_id, hit);
            unit_id
        }
        _ => unit_id,
    };

    let target = world.units.get_unit(victim_id).pos();
    handle_ammo_use(world, action, attack_type, target);
}

/// Handles the result of a ranged attack on terrain.
///
/// # Arguments
///
/// * `pos` - The position of the terrain being attacked.
/// * `world` - The current game world.
/// * `attack_type` - The type of the attack (throw or shoot).
/// * `action` - The action being performed.
fn finish_terrain(pos: Point, world: &mut World, attack_type: AttackType, action: &Action) {
    // TODO: implement terrain attack
    handle_ammo_use(world, action, attack_type, pos);
}

/// Calculates the result of a ranged attack on a unit.
///
/// # Arguments
///
/// * `world` - The current game world.
/// * `unit_id` - The ID of the unit being attacked.
/// * `attack_type` - The type of the attack (throw or shoot).
/// * `action` - The action being performed.
fn get_unit_attack_result(
    world: &mut World,
    unit_id: usize,
    attack_type: AttackType,
    action: &Action,
) -> UnitRangedAttackResult {
    let owner = action.owner(world);
    let unit = world.units.get_unit(unit_id);
    ranged_attack_unit(attack_type, owner.as_fighter(), unit.as_fighter(), world)
}

/// Logs the result of a ranged attack on a unit.
///
/// # Arguments
///
/// * `attack_result` - The result of the attack.
/// * `attack_type` - The type of the attack (throw or shoot).
/// * `world` - The current game world.
/// * `unit_id` - The ID of the unit being attacked.
/// * `owner_id` - The ID of the unit performing the attack.
fn log_unit_attack_result(
    attack_result: &UnitRangedAttackResult,
    attack_type: AttackType,
    world: &mut World,
    unit_id: usize,
    owner_id: usize,
) {
    let owner = world.units.get_unit(owner_id);
    let ends_s = owner.pronouns().verb_ends_with_s();
    let weapon_name = owner.as_fighter().weapon(attack_type).unwrap().name;
    let unit = world.units.get_unit(unit_id);
    let target = unit.pos();
    match attack_result {
        UnitRangedAttackResult::InnocentBystander(victim_id, hit) => {
            let victim = world.units.get_unit(*victim_id);
            let damage = hit.params.damage.to_string();
            for event in unit_attack_success(
                owner,
                unit,
                hit,
                format!(
                    "{} {} {} at {} but miss{} and hit{} {}, dealing {} damage{}.",
                    owner.name_for_actions(),
                    attack_type.verb_a(ends_s),
                    a(weapon_name),
                    unit.name_for_actions(),
                    if ends_s { "es" } else { "" },
                    if ends_s { "s" } else { "" },
                    victim.name_for_actions(),
                    if hit.params.damage == 0 {
                        "no"
                    } else {
                        &damage
                    },
                    if hit.params.penetration > 0 {
                        format!(" and {} armor penetration", hit.params.penetration)
                    } else {
                        String::new()
                    },
                ),
            ) {
                world.log().push(event);
            }
        }
        UnitRangedAttackResult::Miss => {
            world.log().push(LogEvent::warning(
                format!(
                    "{} {} {} at {} but miss{}.",
                    owner.name_for_actions(),
                    attack_type.verb_a(ends_s),
                    a(weapon_name),
                    unit.name_for_actions(),
                    if ends_s { "es" } else { "" },
                ),
                target,
            ));
        }
        UnitRangedAttackResult::Hit(hit) => {
            let damage = hit.params.damage.to_string();
            for event in unit_attack_success(
                owner,
                unit,
                hit,
                format!(
                    "{} {} {} at {} and hit{}, dealing {} damage{}.",
                    owner.name_for_actions(),
                    attack_type.verb_a(ends_s),
                    a(weapon_name),
                    unit.name_for_actions(),
                    if ends_s { "s" } else { "" },
                    if hit.params.damage == 0 {
                        "no"
                    } else {
                        &damage
                    },
                    if hit.params.penetration > 0 {
                        format!(" and {} armor penetration", hit.params.penetration)
                    } else {
                        String::new()
                    },
                ),
            ) {
                world.log().push(event);
            }
        }
        _ => {}
    }
}

/// Handles the use of ammo after a ranged attack.
///
/// # Arguments
///
/// * `world` - The current game world.
/// * `action` - The action being performed.
/// * `attack_type` - The type of the attack (throw or shoot).
/// * `pos` - The position of the attack.
fn handle_ammo_use(world: &mut World, action: &Action, attack_type: AttackType, target: Point) {
    let item = match attack_type {
        AttackType::Throw => action
            .owner_mut(world)
            .inventory_mut()
            .and_then(Inventory::main_hand_take),
        AttackType::Shoot => {
            let owner = action.owner_mut(world);
            if let Some(weapon) = owner.inventory_mut().unwrap().main_hand_mut() {
                if weapon.need_ammo().is_some() {
                    weapon.container_mut().unwrap().items.pop()
                } else {
                    None
                }
            } else {
                None
            }
        }
        AttackType::Melee => unreachable!("Melee attack is not ranged"),
    };

    if let Some(item) = item {
        world.map().get_tile_mut(target).items.push(item);
    }

    let owner = action.owner_mut(world);
    let auto_reload = owner
        .inventory()
        .unwrap()
        .main_hand()
        .is_some_and(|weapon| {
            weapon
                .need_ammo()
                .is_some_and(|need_ammo| need_ammo.reload == 0)
        });
    if auto_reload {
        owner.inventory_mut().unwrap().reload().ok();
    }
}
