use crate::game::game_data::NeedAmmoValue;
use crate::game::{Action, LogEvent};

use super::super::{
    super::{super::lang::a, traits::Name, Avatar, World},
    ActionImpl,
    ActionPossibility::{self, No, Yes},
    ActionType,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Reload {}

impl Reload {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ActionType {
        Self {}.into()
    }
}

impl ActionImpl for Reload {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let actor = world.units.get_unit(actor_id);
        actor
            .inventory()
            .map_or(No("You have no inventory".to_string()), |inventory| {
                inventory.main_hand().map_or(
                    No("You have nothing to reload".to_string()),
                    |weapon| {
                        weapon.need_ammo().map_or(
                            No(format!("Your {} can't be reloaded", weapon.name())),
                            |NeedAmmoValue { typ, reload, .. }| {
                                weapon.container().map_or(
                                    No(format!("Your {} can't be reloaded", weapon.name())),
                                    |container| {
                                        if container.free_volume() > 0 {
                                            if actor.inventory().unwrap().has_ammo(typ) {
                                                Yes(reload as u32)
                                            } else {
                                                No(format!(
                                                    "You don't have ammo for {}!",
                                                    a(weapon.name())
                                                ))
                                            }
                                        } else {
                                            No(format!("Your {} is fully loaded", weapon.name()))
                                        }
                                    },
                                )
                            },
                        )
                    },
                )
            })
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        action
            .owner_mut(world)
            .inventory_mut()
            .unwrap()
            .reload()
            .ok();
        let weapon_name = action
            .owner(world)
            .inventory()
            .unwrap()
            .main_hand()
            .unwrap()
            .name();
        world.log().push(LogEvent::success(
            format!(
                "{} reload{} your {weapon_name}",
                action.owner(world).name_for_actions(),
                if action.owner(world).pronouns().verb_ends_with_s() {
                    "s"
                } else {
                    ""
                }
            ),
            action.owner(world).pos(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{
        QUIVER, WOODEN_ARROW, WOODEN_BOLT, WOODEN_CROSSBOW, WOODEN_SHORTBOW,
    };
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar, Item};

    use super::Reload;

    #[test]
    fn test_cant_reload_without_weapon() {
        let mut world = prepare_world();
        world.units.player_mut().inventory_mut().unwrap().clear();

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_cant_reload_without_ammo() {
        let mut world = prepare_world();
        world.units.player_mut().inventory_mut().unwrap().clear();
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW));

        assert!(Action::new(0, Reload {}.into(), &world).is_err());
    }

    #[test]
    fn test_cant_reload_when_weapon_is_full() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW).with_items_inside([Item::new(WOODEN_ARROW)]));
        world.units.player_mut().inventory_mut().unwrap().wear(
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
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_SHORTBOW));
        world.units.player_mut().inventory_mut().unwrap().wear(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_ARROW)]),
            0,
        );
        assert_eq!(
            world
                .units
                .player()
                .inventory()
                .unwrap()
                .main_hand()
                .unwrap()
                .container()
                .unwrap()
                .items
                .len(),
            0
        );

        let action = Action::new(0, Reload {}.into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(
            world
                .units
                .player()
                .inventory()
                .unwrap()
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
            .inventory_mut()
            .unwrap()
            .wield(Item::new(WOODEN_CROSSBOW));
        world.units.player_mut().inventory_mut().unwrap().wear(
            Item::new(QUIVER).with_items_inside([Item::new(WOODEN_BOLT)]),
            0,
        );
        assert_eq!(
            world
                .units
                .player()
                .inventory()
                .unwrap()
                .main_hand()
                .unwrap()
                .container()
                .unwrap()
                .items
                .len(),
            0
        );

        let action = Action::new(0, Reload {}.into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(
            world
                .units
                .player()
                .inventory()
                .unwrap()
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
