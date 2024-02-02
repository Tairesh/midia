use geometry::Direction;

use crate::game::log::{LogCategory, LogEvent};
use crate::game::map::{TerrainInteract, TerrainView};
use crate::game::traits::Name;
use crate::game::{Action, Avatar, World};

use super::super::{
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Open {
    pub dir: Direction,
}

impl ActionImpl for Open {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);

        if !tile.terrain.can_be_opened() {
            return No(format!("You can't open the {}", tile.terrain.name()));
        }

        Yes(50)
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let units = world.units();
        let owner = action.owner(&units);
        let pos = owner.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile_mut(pos);

        world.log().push(LogEvent::new(
            format!(
                "{} open{} the {}",
                owner.name_for_actions(),
                if owner.pronounce().verb_ends_with_s() {
                    "S"
                } else {
                    ""
                },
                tile.terrain.name()
            ),
            pos,
            LogCategory::Info,
        ));
        let (new_terrain, mut items) = tile.terrain.open();
        tile.terrain = new_terrain;
        tile.items.append(&mut items);

        drop(map);
        drop(units);
        world.calc_fov();
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::{terrains::Chest, TerrainInteract};
    use crate::game::world::tests::prepare_world;
    use crate::game::Action;

    use super::Open;

    #[test]
    fn test_opening() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), false).into();

        let typ = Open {
            dir: Direction::East,
        };
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.units_mut().player_mut().action = Some(action);
            while world.units().player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot open");
        }

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("You open the closed chest"),
            "event: {:?}",
            event
        );
        assert!(world
            .map()
            .get_tile(Point::new(1, 0))
            .terrain
            .can_be_closed());
    }
}
