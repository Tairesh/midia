use std::iter::Cloned;

use geometry::DIR8;
use rand::seq::SliceRandom;

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

// TODO: Shooting should send missiles through entire map when missed target

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Shoot {
    target: usize,
}

impl Shoot {
    pub fn new(target: usize) -> Self {
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

        if !weapon.ammo_types().is_empty() && !actor.wear.has_ammo(weapon.ammo_types()) {
            return No(format!("You have no ammo for {}", a(weapon.name())));
        }

        if let Some(ranged_value) = weapon.ranged_damage() {
            if ranged_value.distance == 0 {
                return No(format!("You can't shoot from {}.", a(weapon.name())));
            }

            let target = world.units.get_unit(self.target);
            if target.is_dead() {
                // This should be unreachable, but just in case.
                return No("You can't shoot to a dead body.".to_string());
            }

            let distance =
                RangedDistance::define(actor.pos.distance(target.pos), ranged_value.distance);
            if distance == RangedDistance::Unreachable {
                return No(format!(
                    "You can't shoot from {} that far.",
                    a(weapon.name())
                ));
            }
            if distance == RangedDistance::Melee {
                return No("You can't shoot in closed combat.".to_string());
            }

            Yes(ATTACK_MOVES)
        } else {
            No(format!("You can't shoot from {}.", weapon.name()))
        }
    }

    // TODO: refactor this, code almost the same as in throw.rs
    #[allow(clippy::too_many_lines)]
    fn on_finish(&self, action: &Action, world: &mut World) {
        let unit = world.units.get_unit(self.target);
        if unit.is_dead() {
            return;
        }
        let target = unit.pos;

        let owner = action.owner(world);
        let weapon_name = owner.wield.main_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );

        let attack_result = ranged_attack_unit(AttackType::Shoot, owner, unit, world);
        match attack_result {
            UnitRangedAttackResult::InnocentBystander(unit_id, hit) => {
                let victim = world.units.get_unit(unit_id);
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

                world.apply_damage(victim.id, hit);
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

                world.apply_damage(unit.id, hit);
            }
            UnitRangedAttackResult::Explosion(_, _) => {
                todo!()
            }
            UnitRangedAttackResult::Impossible => {
                panic!("Impossible ranged attack");
            }
        }

        let owner = action.owner_mut(world);
        if let Some(weapon) = owner.wield.main_hand() {
            if !weapon.ammo_types().is_empty() {
                if let Some(proto) = owner.selected_ammo() {
                    owner.wear.remove_by_proto(&proto);
                } else {
                    owner.wear.remove_ammo(weapon.ammo_types());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use geometry::Point;

    use crate::game::map::items::helpers::{QUIVER, WOODEN_ARROW, WOODEN_SHORTBOW};
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::{Action, Item, ItemPrototype, ItemSize};

    use super::{Shoot, ATTACK_MOVES};

    #[test]
    fn test_shoot_from_bow() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(3, 0));
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units.player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );

        world.units.player_mut().action =
            Some(Action::new(0, Shoot::new(1).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("shoot a wooden short bow at"),
            "msg \"{}\" doesn't contains \"shoot from your wooden short bow to\"",
            event.msg
        );

        assert!(
            Action::new(0, Shoot::new(1).into(), &world).is_err(),
            "Assert we can't shoot second time cause there is no more arrows"
        );
    }

    #[test]
    fn test_cant_shoot_without_weapon() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(3, 0));
        world.units.player_mut().wield.clear();

        assert!(Action::new(0, Shoot::new(1).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(49, 0));
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units.player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );

        assert!(Action::new(0, Shoot::new(1).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_without_arrows() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(3, 0));
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units.player_mut().wear.clear();

        assert!(Action::new(0, Shoot::new(1).into(), &world).is_err());
    }
}
