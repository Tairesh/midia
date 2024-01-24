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
pub struct Close {
    pub dir: Direction,
}

impl ActionImpl for Close {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);

        if !tile.terrain.can_be_closed() {
            return No(format!("You can't close the {}", tile.terrain.name()));
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
        let new_terrain = if tile.terrain.can_suck_items_on_close() {
            tile.terrain
                .close_and_suck_items(tile.items.drain(..).collect())
        } else {
            tile.terrain.close()
        };
        tile.terrain = new_terrain;

        drop(map);
        world.calc_fov();
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::{terrains::Chest, TerrainInteract};
    use crate::game::world::tests::prepare_world;
    use crate::game::Action;

    use super::Close;

    #[test]
    fn test_closing() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), true).into();

        let typ = Close {
            dir: Direction::East,
        };
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.units.player_mut().action = Some(action);
            while world.units.player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot close");
        }

        assert!(world.log().new_events()[0].msg.contains("closed"));
        assert!(world
            .map()
            .get_tile(Point::new(1, 0))
            .terrain
            .can_be_opened());
    }
}
