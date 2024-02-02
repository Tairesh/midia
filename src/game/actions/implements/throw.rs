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

        if actor.inventory.main_hand().is_none() {
            return No("You have nothing to throw.".to_string());
        }

        let item = actor.inventory.main_hand().unwrap();
        if let Some(throw_value) = item.throw_damage() {
            if throw_value.distance == 0 {
                return No(format!("You can't throw {}.", a(item.name())));
            }

            let distance =
                RangedDistance::define(actor.pos.distance(self.target), throw_value.distance);
            if distance == RangedDistance::Unreachable {
                return No(format!("You can't throw {} that far.", a(item.name())));
            }

            let mut map = world.map();
            let units = &map.get_tile(self.target).units;
            if units.is_empty() {
                return Yes(ATTACK_MOVES);
            }

            let unit_id = units.iter().copied().next().unwrap();
            drop(map);

            if distance == RangedDistance::Melee {
                return No(format!(
                    "You can't throw {} in closed combat.",
                    a(item.name())
                ));
            }

            if world.units().get_unit(unit_id).is_dead() {
                // This should be unreachable, but just in case.
                return No(format!(
                    "You can't throw {} at a dead body.",
                    a(item.name())
                ));
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
            drop(map);
            let item = action
                .owner_mut(&mut world.units_mut())
                .inventory
                .main_hand_take()
                .unwrap();
            world.log().push(LogEvent::info(
                format!(
                    "{} throw{} {}.",
                    action.owner(&world.units()).name_for_actions(),
                    if action.owner(&world.units()).pronounce().verb_ends_with_s() {
                        "s"
                    } else {
                        ""
                    },
                    a(item.name()),
                ),
                self.target,
            ));
            world.map().get_tile_mut(self.target).items.push(item);
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
        let weapon_name = owner.inventory.main_hand().unwrap().name();

        let attack_result = ranged_attack_unit(AttackType::Throw, owner, unit, world);
        match attack_result {
            UnitRangedAttackResult::InnocentBystander(unit_id, hit) => {
                let victim = units.get_unit(unit_id);
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

                let victim_id = victim.id;
                drop(units);
                world.apply_damage(victim_id, hit);
                let item = action
                    .owner_mut(&mut world.units_mut())
                    .inventory
                    .main_hand_take()
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
                drop(units);
                let item = action
                    .owner_mut(&mut world.units_mut())
                    .inventory
                    .main_hand_take()
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

                let unit_id = unit.id;
                drop(units);
                world.apply_damage(unit_id, hit);
                let item = action
                    .owner_mut(&mut world.units_mut())
                    .inventory
                    .main_hand_take()
                    .unwrap();
                world.map().get_tile_mut(target).items.push(item);
            }
            UnitRangedAttackResult::Explosion(_, _) => {
                drop(units);
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

    use crate::game::map::items::helpers::ROCK;
    use crate::game::traits::Name;
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::{Action, Item, ItemPrototype, ItemSize};

    use super::{Throw, ATTACK_MOVES};

    #[test]
    fn test_throw_rock() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory
            .wield(Item::new(ROCK));

        world.units_mut().player_mut().action =
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
        world.units_mut().player_mut().inventory.clear();

        assert!(Action::new(0, Throw::new(target).into(), &world).is_err());
    }

    #[test]
    fn test_cant_throw_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(15, 0);
        add_npc(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory
            .wield(Item::new(ROCK));

        assert!(Action::new(0, Throw::new(target).into(), &world).is_err());
    }

    #[test]
    fn test_cant_throw_big_item() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_npc(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory
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
                need_ammo: None,
                is_ammo: None,
            }));

        assert!(Action::new(0, Throw::new(target).into(), &world).is_err());
    }

    // TODO: add test for throwing to terrain
    // TODO: add test for throwing obsidian shards (they should be destroyed)
}
