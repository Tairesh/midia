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
        if actor.char_sheet.shock {
            return No("You are in shock".to_string());
        }

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

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::items::helpers::random_book;
    use crate::game::world::tests::prepare_world;
    use crate::game::Action;

    use super::Read;

    #[test]
    fn test_reading() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(random_book());

        let typ = Read {
            dir: Direction::East,
        };
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.player_mut().action = Some(action);
            while world.player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot read");
        }

        assert!(world.log().new_events()[0].msg.contains("book is called"));
    }
}
