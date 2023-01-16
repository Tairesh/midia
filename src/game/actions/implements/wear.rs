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
                No(format!("You can't wear {}.", item.name()))
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
                    "{} wear{} {}.",
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
