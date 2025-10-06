use roguemetry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        Avatar, RollResult, Skill, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
    ActionType,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Read {
    dir: Direction,
    reading_roll: RollResult,
}

impl Read {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(dir: Direction, avatar: &dyn Avatar) -> ActionType {
        let reading_roll = avatar
            .char_sheet()
            .get_skill_with_modifiers(Skill::Reading)
            .roll_explosive();
        Self { dir, reading_roll }.into()
    }

    #[cfg(test)]
    pub fn new_test(dir: Direction, reading_roll: RollResult) -> ActionType {
        Self { dir, reading_roll }.into()
    }
}

impl ActionImpl for Read {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let actor = world.units.get_unit(actor_id);
        if actor.char_sheet().shock {
            return No("You are in shock".to_string());
        }

        let pos = actor.pos() + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if tile.is_readable() {
            // Every character takes one tick to read, wow!
            let reading_time = tile.read().len() as f32;
            if self.reading_roll.natural == 1 {
                return No("You tried to read it but failed".to_string());
            }
            match self.reading_roll.successes() {
                0 => Yes((reading_time * 2.0).round() as u32),
                1 => Yes(reading_time.round() as u32),
                2.. => Yes((reading_time * 0.5).round() as u32),
            }
        } else {
            No("There is nothing to read".to_string())
        }
    }
    fn on_finish(&self, action: &Action, world: &mut World) {
        let owner = action.owner(world);
        if owner.is_player() {
            let pos = owner.pos() + self.dir;
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
    use roguemetry::{Direction, Point};

    use crate::game::map::items::helpers::book_debug;
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar, RollResult};

    use super::Read;

    #[test]
    fn test_reading() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(book_debug());

        let typ = Read::new_test(Direction::East, RollResult::new(4, 4));
        if let Ok(action) = Action::new(0, typ, &world) {
            assert_eq!(action.length, 52);
            world.units.player_mut().set_action(Some(action));
            while world.units.player().action().is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot read");
        }

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event
                .msg
                .contains("Text on this strange book says «Lore Of The Midia»"),
            "Event: {:?}",
            event
        );
    }

    #[test]
    fn test_cant_read() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(book_debug());

        let typ = Read::new_test(Direction::East, RollResult::new(1, 1));
        assert!(Action::new(0, typ, &world).is_err());
    }

    #[test]
    fn test_slow_read() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(book_debug());

        let typ = Read::new_test(Direction::East, RollResult::new(2, 2));
        let action = Action::new(0, typ, &world).unwrap();
        assert_eq!(action.length, 104);
    }

    #[test]
    fn test_fast_read() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(book_debug());

        let typ = Read::new_test(Direction::East, RollResult::new(8, 8));
        let action = Action::new(0, typ, &world).unwrap();
        assert_eq!(action.length, 26);
    }
}
