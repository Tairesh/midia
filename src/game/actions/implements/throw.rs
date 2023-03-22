use geometry::DIR8;
use rand::seq::SliceRandom;

use super::super::{
    super::{
        log::helpers::unit_attack_success,
        savage::{throw_attack_unit, RangedDistance, UnitRangedAttackResult},
        Action, Avatar, Item, LogEvent, World,
    },
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

const THROW_ATTACK_MOVES: u32 = 10;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Throw {
    target: usize,
}

impl Throw {
    pub fn new(target: usize) -> Self {
        Self { target }
    }
}

impl ActionImpl for Throw {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        if actor.wield.active_hand().is_none() {
            return No("You have nothing to throw.".to_string());
        }

        let item = actor.wield.active_hand().unwrap();
        if let Some(throw_value) = item.throw_damage() {
            if throw_value.distance == 0 {
                return No(format!("You can't throw {}.", item.name()));
            }

            let target = world.get_unit(self.target);
            if target.is_dead() {
                // This should be unreachable, but just in case.
                return No(format!("You can't throw {} at a dead body.", item.name()));
            }

            let distance =
                RangedDistance::define(actor.pos.distance(target.pos), throw_value.distance);
            if distance == RangedDistance::Unreachable {
                return No(format!("You can't throw {} that far.", item.name()));
            }

            Yes(THROW_ATTACK_MOVES)
        } else {
            No(format!("You can't throw {}.", item.name()))
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let unit = world.get_unit(self.target);
        if unit.is_dead() {
            return;
        }
        let target = unit.pos;

        let owner = action.owner(world);
        let weapon_name = owner.wield.active_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );

        let attack = throw_attack_unit(owner, unit, world);
        match attack {
            UnitRangedAttackResult::InnocentBystander(unit_id, damage) => {
                let victim = world.get_unit(unit_id);
                let target = victim.pos;
                for event in unit_attack_success(
                    owner,
                    victim,
                    &damage,
                    format!(
                        "{} throw{} {} {weapon_name} to {} but misses and hit {}{}",
                        owner.name_for_actions(),
                        if owner.is_player() { "" } else { "s" },
                        owner.pronounce().2,
                        unit.name_for_actions(),
                        victim.name_for_actions(),
                        if damage.params.damage == 0 {
                            " but it doesn't do any damage"
                        } else {
                            "!"
                        },
                    ),
                ) {
                    world.log().push(event);
                }

                world.apply_damage(victim.id, damage);
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
                        "{} throw{} {} {weapon_name} to {} and miss.",
                        owner.name_for_actions(),
                        if owner.is_player() { "" } else { "s" },
                        owner.pronounce().2,
                        unit.name_for_actions(),
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
            UnitRangedAttackResult::Hit(damage) => {
                for event in unit_attack_success(
                    owner,
                    unit,
                    &damage,
                    format!(
                        "{} throw{} {} {weapon_name} to {} and hit{}",
                        owner.name_for_actions(),
                        if owner.is_player() { "" } else { "s" },
                        owner.pronounce().2,
                        unit.name_for_actions(),
                        if damage.params.damage == 0 {
                            " but it doesn't do any damage"
                        } else {
                            "!"
                        },
                    ),
                ) {
                    world.log().push(event);
                }

                world.apply_damage(unit.id, damage);
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

    use super::{Throw, THROW_ATTACK_MOVES};

    #[test]
    fn test_throw_rock() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(3, 0));
        world.player_mut().wield.wield(Item::new("rock"));

        world.player_mut().action = Some(Action::new(0, Throw::new(1).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, THROW_ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("throw your rock to"),
            "msg \"{}\" doesn't contains \"throw your rock to\"",
            event.msg
        );
    }

    #[test]
    fn test_cant_throw_without_item() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(3, 0));
        world.player_mut().wield.clear();

        assert!(Action::new(0, Throw::new(1).into(), &world).is_err());
    }

    #[test]
    fn test_cant_throw_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(15, 0));
        world.player_mut().wield.wield(Item::new("rock"));

        assert!(Action::new(0, Throw::new(1).into(), &world).is_err());
    }

    #[test]
    fn test_cant_throw_big_item() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(3, 0));
        world.player_mut().wield.wield(Item::custom(ItemPrototype {
            id: "big_thing".to_string(),
            name: "big thing".to_string(),
            looks_like: "rock".to_string(),
            size: ItemSize::Huge,
            materials: HashSet::new(),
            tags: HashSet::new(),
            qualities: HashSet::new(),
            two_handed_tool: false,
            wearable: None,
            melee_damage: None,
            color_from_material: None,
            throw_damage: None,
        }));

        assert!(Action::new(0, Throw::new(1).into(), &world).is_err());
    }
}
