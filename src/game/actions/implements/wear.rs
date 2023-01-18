use crate::game::LogEvent;

use super::super::{
    super::{Action, Avatar, World},
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Wear {}

impl ActionImpl for Wear {
    fn is_possible(&self, actor: &Avatar, _world: &World) -> ActionPossibility {
        if let Some(item) = actor.wield.active_hand() {
            if item.is_wearable() {
                Yes(1)
            } else {
                No(format!("You can't wear the {}.", item.name()))
            }
        } else {
            No("You don't have anything in your hand.".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        if let Some(item) = action.owner_mut(world).wield.take_from_active_hand() {
            let owner = action.owner(world);
            world.log().push(LogEvent::success(
                format!(
                    "{} put{} on the {}.",
                    owner.name_for_actions(),
                    if owner.is_player() { "" } else { "s" },
                    item.name()
                ),
                owner.pos,
            ));
            action.owner_mut(world).wear.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{axe, cloak};
    use crate::game::world::tests::prepare_world;
    use crate::game::Action;

    use super::Wear;

    #[test]
    fn test_wear() {
        let mut world = prepare_world();
        world.player_mut().wield.wield(cloak());
        world.player_mut().wear.clear();

        if let Ok(action) = Action::new(0, Wear {}.into(), &world) {
            world.player_mut().action = Some(action);
            while world.player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot wear");
        }

        assert!(world.log().new_events()[0].msg.contains("put on the cloak"));
        assert!(world.player().wield.is_empty());
        assert!(world
            .player()
            .wear
            .iter()
            .any(|i| i.proto.id == cloak().proto.id));
    }

    #[test]
    fn test_wear_invalid_items() {
        let mut world = prepare_world();
        world.player_mut().wield.clear();
        assert!(Action::new(0, Wear {}.into(), &world).is_err());

        world.player_mut().wield.wield(axe());
        assert!(Action::new(0, Wear {}.into(), &world).is_err());
    }
}
