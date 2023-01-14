use geometry::Direction;

use crate::game::log::{LogCategory, LogEvent};
use crate::game::map::{TerrainInteract, TerrainView};
use crate::game::{Action, Avatar, World};

use super::super::{
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Close {
    pub dir: Direction,
}

impl ActionImpl for Close {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);

        if !tile.terrain.can_be_closed() {
            return No(format!(
                "{} can't close the {}",
                actor.name_for_actions(),
                tile.terrain.name()
            ));
        }

        Yes(50)
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let owner = action.owner(world);
        let pos = owner.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile_mut(pos);

        world.log().push(LogEvent::new(
            format!(
                "{} closed the {}",
                action.owner(world).name_for_actions(),
                tile.terrain.name()
            ),
            pos,
            LogCategory::Info,
        ));
        let new_terrain = tile.terrain.close();
        tile.terrain = new_terrain;

        drop(map);
        world.calc_fov();
    }
}
