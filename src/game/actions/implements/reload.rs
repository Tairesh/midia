use std::fmt::format;

use crate::game::game_data::NeedAmmoValue;
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
        actor
            .wield
            .main_hand()
            .map_or(No("You have nothing to reload".to_string()), |weapon| {
                weapon.need_ammo().map_or(
                    No(format!("Your {} can't be reloaded", weapon.name())),
                    |NeedAmmoValue { typ, reload, .. }| {
                        weapon.container().map_or(
                            No(format!("Your {} can't be reloaded", weapon.name())),
                            |container| {
                                if container.free_volume() > 0 {
                                    if actor.wear.has_ammo(typ) {
                                        Yes(reload as u32)
                                    } else {
                                        No(format!("You don't have ammo for {}!", a(weapon.name())))
                                    }
                                } else {
                                    No(format!("Your {} is fully loaded", weapon.name()))
                                }
                            },
                        )
                    },
                )
            })
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let mut units = world.units_mut();
        action.owner_mut(&mut units).reload();
        let weapon_name = action.owner(&units).wield.main_hand().unwrap().name();
        world.log().push(LogEvent::success(
            format!(
                "{} reload{} your {weapon_name}",
                action.owner(&units).name_for_actions(),
                if action
                    .owner(&units)
                    .personality
                    .mind
                    .gender
                    .pronounce()
                    .verb_ends_with_s()
                {
                    "s"
                } else {
                    ""
                }
            ),
            action.owner(&units).pos,
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
        let world = prepare_world();
        world.units_mut().player_mut().wield.clear();

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_cant_reload_without_ammo() {
        let world = prepare_world();
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units_mut().player_mut().wear.clear();

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_cant_reload_when_weapon_is_full() {
        let world = prepare_world();
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW).with_items_inside([Item::new(WOODEN_ARROW)]));
        world.units_mut().player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_reload_bow() {
        let mut world = prepare_world();
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units_mut().player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );
        assert_eq!(
            world
                .units()
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
        world.units_mut().player_mut().action = Some(action);
        world.tick();

        assert_eq!(
            world
                .units()
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
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(WOODEN_CROSSBOW));
        world.units_mut().player_mut().wear.add(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_BOLT)]),
            0,
        );
        assert_eq!(
            world
                .units()
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
        world.units_mut().player_mut().action = Some(action);
        world.tick();

        assert_eq!(
            world
                .units()
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
