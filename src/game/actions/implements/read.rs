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
pub struct Read {
    pub dir: Direction,
}

impl ActionImpl for Read {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        // TODO: check skill of reading, and probably even another languages
        let tile = map.get_tile(pos);
        if tile.is_readable() {
            // Every character takes one tick to read, wow!
            Yes(tile.read().len() as u32)
        } else {
            No("There is nothing to read".to_string())
        }
    }
    fn on_finish(&self, action: &Action, world: &mut World) {
        if action.owner(world).is_player() {
            let pos = action.owner(world).pos + self.dir;
            world.log().push(LogEvent::new(
                world.map().get_tile(pos).read(),
                pos,
                LogCategory::Success,
            ));
        }
    }
}
