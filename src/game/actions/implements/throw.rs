use geometry::{Point, DIR8};
use rand::seq::SliceRandom;

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Throw {
    target: Point,
}

impl Throw {
    pub fn new(target: Point) -> Self {
        Self { target }
    }
}

impl ActionImpl for Throw {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        if actor.wield.main_hand().is_none() {
            return No("You have nothing to throw.".to_string());
        }

        let item = actor.wield.main_hand().unwrap();
        if let Some(throw_value) = item.throw_damage() {
            if throw_value.distance == 0 {
                return No(format!("You can't throw {}.", a(item.name())));
            }

            let mut map = world.map();
            let units = &map.get_tile(self.target).units;
            if units.is_empty() {
                // TODO: throw to terrain
                return No("There is no one to throw at.".to_string());
            }
            let unit_id = units.iter().copied().next().unwrap();
            drop(map);

            let target = world.units.get_unit(unit_id);
            if target.is_dead() {
                // This should be unreachable, but just in case.
                return No(format!(
                    "You can't throw {} at a dead body.",
                    a(item.name())
                ));
            }

            let distance =
                RangedDistance::define(actor.pos.distance(target.pos), throw_value.distance);
            if distance == RangedDistance::Unreachable {
                return No(format!("You can't throw {} that far.", a(item.name())));
            }

            Yes(ATTACK_MOVES)
        } else {
            No(format!("You can't throw {}.", a(item.name())))
        }
    }

    // TODO: refactor this, code almost the same as in shoot.rs
    #[allow(clippy::too_many_lines)]
    fn on_finish(&self, action: &Action, world: &mut World) {
        let mut map = world.map();
        let units = &map.get_tile(self.target).units;
        if units.is_empty() {
            return;
        }
        let unit_id = units.iter().copied().next().unwrap();
        drop(map);

        let unit = world.units.get_unit(unit_id);
        if unit.is_dead() {
            return;
        }
        let target = unit.pos;

        let owner = action.owner(world);
        let weapon_name = owner.wield.main_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );

        let attack_result = ranged_attack_unit(AttackType::Throw, owner, unit, world);
        match attack_result {
            UnitRangedAttackResult::InnocentBystander(unit_id, hit) => {
                let victim = world.units.get_unit(unit_id);
                let target = victim.pos;
                let damage = hit.params.damage.to_string();
                for event in unit_attack_success(
                    owner,
                    victim,
                    &hit,
                    format!(
                        "{} throw{} {} at {} but miss{} and hit{} {}, dealing {} damage {}.",
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
                let item = action
                    .owner_mut(world)
                    .wield
                    .take_from_active_hand()
                    .unwrap();
                world.map().get_tile_mut(target).items.push(item);
            }
            UnitRangedAttackResult::Miss => {
                world.log().push(LogEvent::warning(
                    format!(
                        "{} throw{} {} at {} but miss{}.",
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
                let item = action
                    .owner_mut(world)
                    .wield
                    .take_from_active_hand()
                    .unwrap();
                let random_pos = target + *DIR8.choose(&mut rand::thread_rng()).unwrap();
                world.map().get_tile_mut(random_pos).items.push(item);
            }
            UnitRangedAttackResult::Hit(hit) => {
                let damage = hit.params.damage.to_string();
                for event in unit_attack_success(
                    owner,
                    unit,
                    &hit,
                    format!(
                        "{} throw{} {} at {} and hit{}, dealing {} damage{}.",
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
                let item = action
                    .owner_mut(world)
                    .wield
                    .take_from_active_hand()
                    .unwrap();
                world.map().get_tile_mut(target).items.push(item);
            }
            UnitRangedAttackResult::Explosion(_, _) => {
                todo!()
            }
            UnitRangedAttackResult::Impossible => {
                panic!("Impossible throw attack");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use geometry::Point;

    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::{Action, Item, ItemPrototype, ItemSize};

    use super::{Throw, ATTACK_MOVES};

    #[test]
    fn test_throw_rock() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world.units.player_mut().wield.wield(Item::new("rock"));

        world.units.player_mut().action =
            Some(Action::new(0, Throw::new(target).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("throw a rock at"),
            "msg \"{}\" doesn't contains \"throw your rock to\"",
            event.msg
        );
    }

    #[test]
    fn test_cant_throw_without_item() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world.units.player_mut().wield.clear();

        assert!(Action::new(0, Throw::new(target).into(), &world).is_err());
    }

    #[test]
    fn test_cant_throw_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(15, 0);
        add_npc(&mut world, target);
        world.units.player_mut().wield.wield(Item::new("rock"));

        assert!(Action::new(0, Throw::new(target).into(), &world).is_err());
    }

    #[test]
    fn test_cant_throw_big_item() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world
            .units
            .player_mut()
            .wield
            .wield(Item::custom(ItemPrototype {
                id: "big_thing".to_string(),
                name: "big thing".to_string(),
                looks_like: "rock".to_string(),
                size: ItemSize::Huge,
                materials: HashSet::new(),
                tags: HashSet::new(),
                qualities: Vec::new(),
                two_handed_tool: false,
                wearable: None,
                melee_damage: None,
                color_from_material: None,
                throw_damage: None,
                ranged_damage: None,
                ammo_types: HashSet::new(),
                ammo: None,
            }));

        assert!(Action::new(0, Throw::new(target).into(), &world).is_err());
    }

    // TODO: add test for throwing to terrain
    // TODO: add test for throwing obsidian shards (they should be destroyed)
}
