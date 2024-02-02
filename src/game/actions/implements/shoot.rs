use std::iter::Cloned;

use geometry::{Point, DIR8};
use rand::seq::SliceRandom;

use crate::game::game_data::NeedAmmoValue;
use crate::game::savage::HitResult;
use crate::game::traits::Name;

use super::super::{
    super::{
        super::lang::a,
        log::helpers::unit_attack_success,
        savage::{ranged_attack_unit, RangedDistance, UnitRangedAttackResult, ATTACK_MOVES},
        Action, AttackType, Avatar, Item, LogEvent, World,
    },
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

// TODO: Shooting should send missiles through entire map when there is no obstacles.

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Shoot {
    target: Point,
}

impl Shoot {
    pub fn new(target: Point) -> Self {
        Self { target }
    }
}

impl ActionImpl for Shoot {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        if actor.wield.main_hand().is_none() {
            return No("You have nothing to shoot from.".to_string());
        }

        let weapon = actor.wield.main_hand().unwrap();

        if let Some(NeedAmmoValue { typ, .. }) = weapon.need_ammo() {
            if !weapon.has_ammo(typ) {
                return No(format!("You have no ammo in {}!", a(weapon.name())));
            }
        }

        if let Some(ranged_value) = weapon.ranged_damage() {
            if ranged_value.distance == 0 {
                return No(format!("You can't shoot from {}.", a(weapon.name())));
            }

            let mut map = world.map();
            let units = &map.get_tile(self.target).units;
            if units.is_empty() {
                // TODO: shoot obstacles
                return No("There is no one to shoot.".to_string());
            }
            let unit_id = units.iter().copied().next().unwrap();
            drop(map);

            let units = world.units();
            let target = units.get_unit(unit_id);
            if target.is_dead() {
                // This should be unreachable, but just in case.
                return No("You can't shoot to a dead body.".to_string());
            }

            let distance =
                RangedDistance::define(actor.pos.distance(target.pos), ranged_value.distance);

            match distance {
                RangedDistance::Unreachable => {
                    No(format!("You can't shoot {} that far.", a(weapon.name())))
                }
                RangedDistance::Melee => No("You can't shoot in closed combat.".to_string()),
                _ => Yes(ATTACK_MOVES),
            }
        } else {
            No(format!("You can't shoot from {}.", weapon.name()))
        }
    }

    // TODO: refactor this, code almost the same as in throw.rs
    #[allow(clippy::too_many_lines)]
    fn on_finish(&self, action: &Action, world: &mut World) {
        let mut map = world.map();
        let units = &map.get_tile(self.target).units;
        if units.is_empty() {
            return;
        }
        let unit_id = units.iter().copied().next().unwrap();
        drop(map);

        let units = world.units();
        let unit = units.get_unit(unit_id);
        if unit.is_dead() {
            return;
        }
        let target = unit.pos;

        let owner = action.owner(&units);
        let weapon_name = owner.wield.main_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );

        let attack_result = ranged_attack_unit(AttackType::Shoot, owner, unit, world);
        match attack_result {
            UnitRangedAttackResult::InnocentBystander(unit_id, hit) => {
                let victim = units.get_unit(unit_id);
                let damage = hit.params.damage.to_string();
                for event in unit_attack_success(
                    owner,
                    victim,
                    &hit,
                    format!(
                        "{} shoot{} {} at {} but miss{} and hit{} {}, dealing {} damage{}.",
                        owner.name_for_actions(),
                        if owner.pronounce().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        a(weapon_name),
                        unit.name_for_actions(),
                        if owner.pronounce().verb_ends_with_s() {
                            "es"
                        } else {
                            ""
                        },
                        if owner.pronounce().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
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

                let victim_id = victim.id;
                drop(units);
                world.apply_damage(victim_id, hit);
            }
            UnitRangedAttackResult::Miss => {
                world.log().push(LogEvent::warning(
                    format!(
                        "{} shoot{} {} at {} but miss{}.",
                        owner.name_for_actions(),
                        if owner.pronounce().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        a(weapon_name),
                        unit.name_for_actions(),
                        if owner.pronounce().verb_ends_with_s() {
                            "es"
                        } else {
                            ""
                        },
                    ),
                    target,
                ));
                drop(units);
            }
            UnitRangedAttackResult::Hit(hit) => {
                let damage = hit.params.damage.to_string();
                for event in unit_attack_success(
                    owner,
                    unit,
                    &hit,
                    format!(
                        "{} shoot{} {} at {} and hit{}, dealing {} damage{}.",
                        owner.name_for_actions(),
                        if owner.pronounce().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        a(weapon_name),
                        unit.name_for_actions(),
                        if owner.pronounce().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
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

                let unit_id = unit.id;
                drop(units);
                world.apply_damage(unit_id, hit);
            }
            UnitRangedAttackResult::Explosion(_, _) => {
                drop(units);
                todo!();
            }
            UnitRangedAttackResult::Impossible => {
                panic!("Impossible ranged attack");
            }
        }

        let mut units = world.units_mut();
        let owner = action.owner_mut(&mut units);
        if let Some(weapon) = owner.wield.main_hand_mut() {
            if weapon.need_ammo().is_some() {
                weapon.container_mut().unwrap().items.pop();
            }
        }
        let auto_reload = owner.wield.main_hand().map_or(false, |weapon| {
            weapon
                .need_ammo()
                .map_or(false, |need_ammo| need_ammo.reload == 0)
        });
        if auto_reload {
            owner.reload();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use geometry::Point;

    use crate::game::map::items::helpers::{
        QUIVER, WOODEN_ARROW, WOODEN_BOLT, WOODEN_CROSSBOW, WOODEN_SHORTBOW,
    };
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::{Action, Item, ItemPrototype, ItemSize};

    use super::{Shoot, ATTACK_MOVES};

    #[test]
    fn test_shoot_from_bow() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units_mut().player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );

        // Can't shoot before loading arrow to bow.
        assert!(Action::new(0, Shoot::new(target).into(), &world).is_err());
        world.units_mut().player_mut().reload();

        world.units_mut().player_mut().action =
            Some(Action::new(0, Shoot::new(target).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("shoot a wooden short bow at"),
            "msg \"{}\" doesn't contains \"shoot from your wooden short bow at\"",
            event.msg
        );

        assert!(
            Action::new(0, Shoot::new(target).into(), &world).is_err(),
            "Assert we can't shoot second time cause there is no more arrows"
        );
    }

    #[test]
    fn test_cant_shoot_without_weapon() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world.units_mut().player_mut().wield.clear();

        assert!(Action::new(0, Shoot::new(target).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units_mut().player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );
        world.units_mut().player_mut().reload();

        // Distance of wooden shortbow is 12 so we can shoot to 12*4=48 tiles.
        let target_far = Point::new(48, 0);
        add_npc(&mut world, target_far);
        assert!(Action::new(0, Shoot::new(target_far).into(), &world).is_ok());

        let target_too_far = Point::new(49, 0);
        add_npc(&mut world, target_too_far);
        assert!(Action::new(0, Shoot::new(target_too_far).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_without_arrows() {
        let mut world = prepare_world();

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units_mut().player_mut().wear.clear();
        world.units_mut().player_mut().reload();

        assert!(Action::new(0, Shoot::new(target).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_crossbow_without_reloading() {
        // TODO: reloading action
        let mut world = prepare_world();
        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_CROSSBOW));
        world.units_mut().player_mut().wear.add(
            Item::new(QUIVER).with_items_inside(vec![Item::new(WOODEN_BOLT); 10]),
            0,
        );
        world.units_mut().player_mut().reload();

        world.units_mut().player_mut().action =
            Some(Action::new(0, Shoot::new(target).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("shoot a wooden crossbow at"),
            "msg \"{}\" doesn't contains \"shoot from your wooden crossbow at\"",
            event.msg
        );

        assert!(
            Action::new(0, Shoot::new(target).into(), &world).is_err(),
            "Assert we can't shoot second time cause there is no more bolts in a crossbow"
        );
    }
}
