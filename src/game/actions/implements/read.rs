use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        Avatar, RollResult, Skill, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Read {
    dir: Direction,
    reading_roll: RollResult,
}

impl Read {
    pub fn new(dir: Direction, avatar: &Avatar) -> Self {
        let reading_roll = avatar
            .personality
            .char_sheet
            .get_skill_with_modifiers(Skill::Reading)
            .roll_explosive();
        Self { dir, reading_roll }
    }

    #[cfg(test)]
    pub fn new_test(dir: Direction, reading_roll: RollResult) -> Self {
        Self { dir, reading_roll }
    }
}

impl ActionImpl for Read {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if tile.is_readable() {
            // Every character takes one tick to read, wow!
            let reading_time = tile.read().len() as f32;
            if self.reading_roll.natural == 1 {
                return No("You tried to read it but failed".to_string());
            }
            match self.reading_roll.total {
                ..=3 => Yes((reading_time * 2.0).round() as u32),
                4..=7 => Yes(reading_time.round() as u32),
                8.. => Yes((reading_time * 0.5).round() as u32),
            }
        } else {
            No("There is nothing to read".to_string())
        }
    }
    fn on_finish(&self, action: &Action, world: &mut World) {
        let units = world.units();
        let owner = action.owner(&units);
        if owner.is_player() {
            let pos = owner.pos + self.dir;
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
    use crate::game::{Action, RollResult};

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

        let typ = Read::new_test(Direction::East, RollResult::new(4, 4));
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.units_mut().player_mut().action = Some(action);
            while world.units().player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot read");
        }

        assert!(world.log().new_events()[0].msg.contains("book is called"));
    }

    #[test]
    fn test_cant_read() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(random_book());

        let typ = Read::new_test(Direction::East, RollResult::new(1, 1));
        assert!(Action::new(0, typ.into(), &world).is_err());
    }
}
