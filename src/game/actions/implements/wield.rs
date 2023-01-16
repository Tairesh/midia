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
        let pos = actor.pos + self.dir;
        if let Some(item) = world.map().get_tile(pos).items.last() {
            match world.player().wield.can_wield(item.is_two_handed()) {
                Ok(_) => Yes(item.wield_time().round() as u32),
                Err(e) => No(e),
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
                    "{} wield{} the {}",
                    action.owner(world).name_for_actions(),
                    if action.owner(world).is_player() {
                        ""
                    } else {
                        "s"
                    },
                    name
                ),
                pos,
                LogCategory::Success,
            ));
        }
    }
}
