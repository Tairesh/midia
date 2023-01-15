use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
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
        if !actor.wield.has_free_space() {
            return No(format!(
                "You already have {} in hands",
                world.player().wield.names()
            ));
        }
        let pos = actor.pos + self.dir;
        if let Some(item) = world.map().get_tile(pos).items.last() {
            if actor.wield.can_wield(item) {
                Yes(item.wield_time().round() as u32)
            } else {
                No(format!(
                    "You can't wield {} with {}",
                    item.name(),
                    actor.wield.names()
                ))
            }
        } else {
            No("There is nothing to pick up".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let pos = action.owner(world).pos + self.dir;
        let item = world.map().get_tile_mut(pos).items.pop();
        if let Some(item) = item {
            let name = item.name().to_string();
            action.owner_mut(world).wield.wield(item);
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
