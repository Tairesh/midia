use geometry::{Point, DIR8};
use rand::seq::SliceRandom;

use super::super::{
    super::{
        super::lang::a,
        log::helpers::unit_attack_success,
        savage::{ranged_attack_unit, RangedDistance, UnitRangedAttackResult, ATTACK_MOVES},
        traits::Name,
        Action, AttackType, Avatar, LogEvent, World,
    },
    implements::ranged_attack,
    ActionImpl,
    ActionPossibility::{self, No, Yes},
    ActionType, AttackTarget,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Throw {
    target: AttackTarget,
}

impl Throw {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(target: Point, world: &World) -> ActionType {
        Self {
            target: AttackTarget::auto(target, world),
        }
        .into()
    }
}

impl ActionImpl for Throw {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        ranged_attack::is_possible(actor_id, self.target, world, AttackType::Throw)
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        ranged_attack::on_finish(action, self.target, world, AttackType::Throw);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use geometry::{Direction, Point};

    use crate::assets::Sprite;
    use crate::game::actions::implements::Skip;
    use crate::game::actions::AttackTarget;
    use crate::game::map::items::helpers::ROCK;
    use crate::game::world::tests::{add_dummy, add_monster, prepare_world};
    use crate::game::{Action, Avatar, Item, ItemPrototype, ItemSize};

    use super::*;

    #[test]
    fn test_throw_rock() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(ROCK));

        let action = Action::new(0, Throw::new(target, &world), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
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
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .clear();

        assert!(Action::new(0, Throw::new(target, &world), &world).is_err());
    }

    #[test]
    fn test_cant_throw_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(15, 0);
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(ROCK));

        assert!(Action::new(0, Throw::new(target, &world), &world).is_err());
    }

    #[test]
    fn test_cant_throw_big_item() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::custom(ItemPrototype {
                id: "big_thing".to_string(),
                name: "big thing".to_string(),
                looks_like: Sprite::Rock,
                size: ItemSize::Huge,
                materials: HashSet::new(),
                qualities: Vec::new(),
                two_handed: false,
                wearable: None,
                melee_damage: None,
                color_from_material: None,
                throw_damage: None,
                ranged_damage: None,
                need_ammo: None,
                is_ammo: None,
            }));

        assert!(Action::new(0, Throw::new(target, &world), &world).is_err());
    }

    #[test]
    fn test_throwing_at_moving_target() {
        let mut world = prepare_world();
        let target = Point::new(3, 0);
        let mut units = world.units_mut();
        let player = units.player_mut();
        let inventory = player.inventory_mut().unwrap();
        inventory.wield(Item::new(ROCK));
        drop(units);

        let monster = add_monster(&mut world, target);

        // Wait 5 ticks to make sure monster will move.
        let action = Action::new(0, Skip::new(5), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        let action = Throw::new(target, &world);
        if let ActionType::Throw(action) = action {
            if let AttackTarget::Avatar(unit_id) = action.target {
                assert_eq!(monster, unit_id);
            } else {
                panic!("Unexpected target: {:?}", action.target);
            }
        } else {
            panic!("Unexpected action: {:?}", action);
        }
        let action = Action::new(0, action, &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("throw a rock at Old Bugger"),
            "msg \"{}\" doesn't contains \"throw a rock at Old Bugger\"",
            event.msg
        );
        drop(log);
        assert_eq!(
            world.units().get_unit(monster).pos(),
            target + Direction::West
        );
    }

    // TODO: add test for throwing to terrain
    // TODO: add test for throwing obsidian shards (they should be destroyed)
}
