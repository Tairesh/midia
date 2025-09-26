use rand::seq::SliceRandom;
use roguemetry::Point;

use super::super::{
    super::{
        super::lang::a,
        game_data::NeedAmmoValue,
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

// TODO: Shooting should send missiles through entire map when there is no obstacles.

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Shoot {
    target: AttackTarget,
}

impl Shoot {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(pos: Point, world: &World) -> ActionType {
        Self {
            target: AttackTarget::auto(pos, world),
        }
        .into()
    }
}

impl ActionImpl for Shoot {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        ranged_attack::is_possible(actor_id, self.target, world, AttackType::Shoot)
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        ranged_attack::on_finish(action, self.target, world, AttackType::Shoot);
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::{Direction, Point};

    use crate::game::actions::implements::{Skip, Walk};
    use crate::game::actions::AttackTarget;
    use crate::game::map::items::helpers::{
        QUIVER, WOODEN_ARROW, WOODEN_BOLT, WOODEN_CROSSBOW, WOODEN_SHORTBOW,
    };
    use crate::game::world::tests::{add_dummy, add_monster, prepare_world};
    use crate::game::{Action, Avatar, Item};

    use super::*;

    #[test]
    fn test_shoot_from_bow() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        let target = Point::new(3, 0);
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW));
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wear(
                Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
                0,
            );

        // Can't shoot before loading arrow to bow.
        assert!(Action::new(0, Shoot::new(target, &world).into(), &world).is_err());
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .reload()
            .ok();

        let action = Action::new(0, Shoot::new(target, &world).into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event
                .msg
                .contains("shoot from a wooden short bow (wooden arrow) at"),
            "msg \"{}\" doesn't contains \"shoot from a wooden short bow (wooden arrow) at\"",
            event.msg
        );

        assert!(
            Action::new(0, Shoot::new(target, &world).into(), &world).is_err(),
            "Assert we can't shoot second time cause there is no more arrows"
        );
    }

    #[test]
    fn test_cant_shoot_without_weapon() {
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

        assert!(Action::new(0, Shoot::new(target, &world).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_too_far() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW));
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wear(
                Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
                0,
            );
        world.units_mut().player_mut().inventory.reload().ok();

        // Distance of wooden shortbow is 12 so we can shoot to 12*4=48 tiles.
        let target_far = Point::new(48, 0);
        add_dummy(&mut world, target_far);
        assert!(Action::new(0, Shoot::new(target_far, &world).into(), &world).is_ok());

        let target_too_far = Point::new(49, 0);
        add_dummy(&mut world, target_too_far);
        assert!(Action::new(0, Shoot::new(target_too_far, &world).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_without_arrows() {
        let mut world = prepare_world();

        let target = Point::new(3, 0);
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .clear();
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW));
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .reload()
            .ok();

        assert!(Action::new(0, Shoot::new(target, &world).into(), &world).is_err());
    }

    #[test]
    fn test_cant_shoot_crossbow_without_reloading() {
        let mut world = prepare_world();
        let target = Point::new(3, 0);
        add_dummy(&mut world, target);
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_CROSSBOW));
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wear(
                Item::new(QUIVER).with_items_inside(vec![Item::new(WOODEN_BOLT); 10]),
                0,
            );
        assert!(world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .reload()
            .is_ok());

        let action = Action::new(0, Shoot::new(target, &world).into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event
                .msg
                .contains("shoot from a wooden crossbow (wooden bolt)"),
            "msg \"{}\" doesn't contains \"shoot from a wooden crossbow (wooden bolt)\"",
            event.msg
        );

        assert!(
            Action::new(0, Shoot::new(target, &world).into(), &world).is_err(),
            "Assert we can't shoot second time cause there is no more bolts in a crossbow"
        );
    }

    #[test]
    fn test_shooting_at_moving_target() {
        let mut world = prepare_world();
        let target = Point::new(5, 0);
        let mut units = world.units_mut();
        let player = units.player_mut();
        let inventory = player.inventory_mut().unwrap();
        inventory.wield(Item::new(WOODEN_CROSSBOW));
        inventory.wear(
            Item::new(QUIVER).with_items_inside(vec![Item::new(WOODEN_BOLT); 10]),
            0,
        );
        assert!(inventory.reload().is_ok());

        drop(units);
        let monster = add_monster(&mut world, target);
        let action = Action::new(monster, Walk::new(Direction::West), &world).unwrap();
        world
            .units_mut()
            .get_unit_mut(monster)
            .set_action(Some(action));

        // Wait 5 ticks to make sure monster will move.
        let action = Action::new(0, Skip::new(5).into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        let action = Shoot::new(target, &world);
        if let ActionType::Shoot(shoot) = action {
            if let AttackTarget::Avatar(unit_id) = shoot.target {
                assert_eq!(monster, unit_id);
            } else {
                panic!("Unexpected target: {:?}", shoot.target);
            }
        } else {
            panic!("Unexpected action: {:?}", action);
        }
        let action = Action::new(0, action.into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event
                .msg
                .contains("shoot from a wooden crossbow (wooden bolt)"),
            "msg \"{}\" doesn't contains \"shoot from a wooden crossbow (wooden bolt)\"",
            event.msg
        );
        drop(log);
        assert_eq!(
            world.units().get_unit(monster).pos(),
            target + Direction::West
        );
    }
}
