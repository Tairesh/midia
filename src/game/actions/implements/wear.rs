use crate::game::traits::Name;
use crate::game::LogEvent;

use super::super::{
    super::{Action, Avatar, World},
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

// TODO: variants
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Wear {}

impl ActionImpl for Wear {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let units = world.units();
        let actor = units.get_unit(actor_id);
        if actor.char_sheet().shock {
            return No("You are in shock".to_string());
        }

        if actor.inventory().is_none() {
            return No("You don't have an inventory".to_string());
        }
        if actor.inventory().unwrap().main_hand().is_none() {
            return No("You don't have anything in your hand.".to_string());
        }
        let item = actor.inventory().unwrap().main_hand().unwrap();
        if item.proto().wearable.is_none() {
            return No(format!("You can't wear the {}.", item.name()));
        }
        let wearable = item.proto().wearable.as_ref().unwrap();
        for variant in 0..wearable.variants.len() {
            if actor.inventory().unwrap().can_wear(item, variant) {
                return Yes(10);
            }
        }

        No(format!(
            "You can't wear the {} with armor you already wearing.",
            item.name()
        ))
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let mut units = world.units_mut();
        if let Some(item) = action
            .owner_mut(&mut units)
            .inventory_mut()
            .unwrap()
            .main_hand_take()
        {
            let owner = action.owner(&units);
            world.log().push(LogEvent::success(
                format!(
                    "{} put{} on the {}.",
                    owner.name_for_actions(),
                    if owner.is_player() { "" } else { "s" },
                    item.name()
                ),
                owner.pos(),
            ));
            // TODO: use variant
            for variant in 0..item.proto().wearable.as_ref().unwrap().variants.len() {
                if owner.inventory().unwrap().can_wear(&item, variant) {
                    action
                        .owner_mut(&mut units)
                        .inventory_mut()
                        .unwrap()
                        .wear(item, variant);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{CLOAK, GOD_AXE};
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar, Item};

    use super::Wear;

    #[test]
    fn test_wear() {
        let mut world = prepare_world();
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
            .wield(Item::new(CLOAK));

        if let Ok(action) = Action::new(0, Wear {}.into(), &world) {
            world.units_mut().player_mut().set_action(Some(action));
            while world.units().player().action().is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot wear");
        }

        assert!(world.log().new_events()[0].msg.contains("put on the cloak"));
        assert!(world
            .units()
            .player()
            .inventory()
            .unwrap()
            .main_hand()
            .is_none());
        assert!(world
            .units()
            .player()
            .inventory()
            .unwrap()
            .iter_wear()
            .any(|i| i.proto().id == "cloak"));
    }

    #[test]
    fn test_wear_invalid_items() {
        let world = prepare_world();
        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .clear();
        assert!(Action::new(0, Wear {}.into(), &world).is_err());

        world
            .units_mut()
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(GOD_AXE));
        assert!(Action::new(0, Wear {}.into(), &world).is_err());
    }
}
