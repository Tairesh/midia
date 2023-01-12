use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{ItemInteract, ItemView},
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Wield {
    pub dir: Direction,
}

impl ActionImpl for Wield {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if !actor.wield.is_empty() {
            return No("You already have something in your hands".to_string());
        }
        let pos = actor.pos + self.dir;
        if let Some(item) = world.map().get_tile(pos).items.last() {
            Yes(item.wield_time(actor).round() as u32)
        } else {
            No("There is nothing to pick up".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let pos = action.owner(world).pos + self.dir;
        let item = world.map().get_tile_mut(pos).items.pop();
        if let Some(item) = item {
            let name = item.name();
            action.owner_mut(world).wield.push(item);
            world.log().push(LogEvent::new(
                format!(
                    "{} wield the {}",
                    action.owner(world).name_for_actions(),
                    name
                ),
                pos,
                LogCategory::Success,
            ));
        }
    }
}
