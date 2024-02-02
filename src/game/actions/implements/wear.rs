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
    fn is_possible(&self, actor: &Avatar, _world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        if let Some(item) = actor.inventory.main_hand() {
            if let Some(wearable) = &item.proto().wearable {
                for variant in 0..wearable.variants.len() {
                    if actor.inventory.can_wear(item, variant) {
                        return Yes(10);
                    }
                }

                No(format!(
                    "You can't wear the {} with armor you already wearing.",
                    item.name()
                ))
            } else {
                No(format!("You can't wear the {}.", item.name()))
            }
        } else {
            No("You don't have anything in your hand.".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let mut units = world.units_mut();
        if let Some(item) = action.owner_mut(&mut units).inventory.main_hand_take() {
            let owner = action.owner(&units);
            world.log().push(LogEvent::success(
                format!(
                    "{} put{} on the {}.",
                    owner.name_for_actions(),
                    if owner.is_player() { "" } else { "s" },
                    item.name()
                ),
                owner.pos,
            ));
            // TODO: use variant
            for variant in 0..item.proto().wearable.as_ref().unwrap().variants.len() {
                if owner.inventory.can_wear(&item, variant) {
                    action.owner_mut(&mut units).inventory.wear(item, variant);
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
    use crate::game::{Action, Item};

    use super::Wear;

    #[test]
    fn test_wear() {
        let mut world = prepare_world();
        world.units_mut().player_mut().inventory.clear();
        world
            .units_mut()
            .player_mut()
            .inventory
            .wield(Item::new(CLOAK));

        if let Ok(action) = Action::new(0, Wear {}.into(), &world) {
            world.units_mut().player_mut().action = Some(action);
            while world.units().player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot wear");
        }

        assert!(world.log().new_events()[0].msg.contains("put on the cloak"));
        assert!(world.units().player().inventory.main_hand().is_none());
        assert!(world
            .units()
            .player()
            .inventory
            .iter_wear()
            .any(|i| i.proto().id == "cloak"));
    }

    #[test]
    fn test_wear_invalid_items() {
        let world = prepare_world();
        world.units_mut().player_mut().inventory.clear();
        assert!(Action::new(0, Wear {}.into(), &world).is_err());

        world
            .units_mut()
            .player_mut()
            .inventory
            .wield(Item::new(GOD_AXE));
        assert!(Action::new(0, Wear {}.into(), &world).is_err());
    }
}
