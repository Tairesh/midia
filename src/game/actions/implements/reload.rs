use crate::game::{Action, LogEvent};

use super::super::{
    super::{super::lang::a, traits::Name, Avatar, World},
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Reload {}

impl ActionImpl for Reload {
    fn is_possible(&self, actor: &Avatar, _world: &World) -> ActionPossibility {
        if actor.wield.main_hand().map_or(false, |weapon| {
            weapon.need_ammo().is_some()
                && weapon.container().map_or(false, |container| {
                    container.is_for_ammo() && container.free_volume() > 0
                })
        }) {
            let weapon = actor.wield.main_hand().unwrap();
            let need_ammo = weapon.need_ammo().unwrap();

            if !actor.wear.has_ammo(need_ammo.typ) {
                return No(format!("You don't have ammo for {}", a(weapon.name())));
            }

            Yes(need_ammo.reload as u32)
        } else {
            No("You can't reload".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        action.owner_mut(world).reload();
        let weapon_name = action.owner(world).wield.main_hand().unwrap().name();
        world.log().push(LogEvent::success(
            format!("You reload your {weapon_name}"),
            action.owner(world).pos,
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{
        QUIVER, WOODEN_ARROW, WOODEN_BOLT, WOODEN_CROSSBOW, WOODEN_SHORTBOW,
    };
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Item};

    use super::Reload;

    #[test]
    fn test_cant_reload_without_weapon() {
        let mut world = prepare_world();
        world.units.player_mut().wield.clear();

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_cant_reload_without_ammo() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units.player_mut().wear.clear();

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_cant_reload_when_weapon_is_full() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW).with_items_inside([Item::new(WOODEN_ARROW)]));
        world.units.player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_reload_bow() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units.player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );
        assert_eq!(
            world
                .units
                .player()
                .wield
                .main_hand()
                .unwrap()
                .container()
                .unwrap()
                .items
                .len(),
            0
        );

        let action = Action::new(0, Reload {}.into(), &world).unwrap();
        world.units.player_mut().action = Some(action);
        world.tick();

        assert_eq!(
            world
                .units
                .player()
                .wield
                .main_hand()
                .unwrap()
                .container()
                .unwrap()
                .items
                .len(),
            1
        );
        assert_eq!(world.meta.current_tick, 0);
    }

    #[test]
    fn test_reload_crossbow() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_CROSSBOW));
        world.units.player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_BOLT)]),
            0,
        );
        assert_eq!(
            world
                .units
                .player()
                .wield
                .main_hand()
                .unwrap()
                .container()
                .unwrap()
                .items
                .len(),
            0
        );

        let action = Action::new(0, Reload {}.into(), &world).unwrap();
        world.units.player_mut().action = Some(action);
        world.tick();

        assert_eq!(
            world
                .units
                .player()
                .wield
                .main_hand()
                .unwrap()
                .container()
                .unwrap()
                .items
                .len(),
            1
        );
        assert_eq!(world.meta.current_tick, 10);
    }
}
