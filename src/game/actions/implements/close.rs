use geometry::Direction;

use crate::game::log::{LogCategory, LogEvent};
use crate::game::map::{TerrainInteract, TerrainInteractAction, TerrainView};
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
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let units = world.units();
        let actor = units.get_unit(actor_id);
        let pos = actor.pos() + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);

        if !tile.terrain.supports_action(TerrainInteractAction::Close) {
            return No(format!("You can't close the {}", tile.terrain.name()));
        }

        if !tile.items.is_empty() && !tile.terrain.can_contain_items() {
            let item = &tile.items[0];
            let count = tile.items.len();
            return No(format!(
                "You can't close the {} with {} {}{} on it",
                tile.terrain.name(),
                item.name(),
                if count > 1 { "and other items" } else { "" },
                if count > 1 {
                    format!(" (x{count})")
                } else {
                    String::new()
                }
            ));
        }

        Yes(50)
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let units = world.units();
        let owner = action.owner(&units);
        let pos = owner.pos() + self.dir;
        let mut map = world.map();
        let tile = map.get_tile_mut(pos);

        world.log().push(LogEvent::new(
            format!(
                "{} close{} the {}",
                owner.name_for_actions(),
                if owner.pronouns().verb_ends_with_s() {
                    "s"
                } else {
                    ""
                },
                tile.terrain.name()
            ),
            pos,
            LogCategory::Info,
        ));
        let new_terrain = tile.terrain.close(tile.items.drain(..).collect());
        tile.terrain = new_terrain;

        drop(map);
        drop(units);
        world.calc_fov();
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::items::helpers::WOODEN_SPLINTER;
    use crate::game::map::terrains::Chest;
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar, Item, Terrain, TerrainInteract, TerrainInteractAction};

    use super::Close;

    #[test]
    fn test_closing() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), true).into();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(Item::new(WOODEN_SPLINTER));

        let typ = Close {
            dir: Direction::East,
        };
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.units_mut().player_mut().set_action(Some(action));
            while world.units().player().action().is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot close");
        }

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("You close the chest"),
            "event: {:?}",
            event
        );
        assert!(world
            .map()
            .get_tile(Point::new(1, 0))
            .terrain
            .supports_action(TerrainInteractAction::Open));
        assert!(world.map().get_tile(Point::new(1, 0)).items.is_empty());
        if let Terrain::Chest(Chest { items_inside, open }) =
            &world.map().get_tile(Point::new(1, 0)).terrain
        {
            assert!(!open);
            assert_eq!(items_inside.len(), 1);
            assert_eq!(items_inside[0].proto().id, WOODEN_SPLINTER);
        } else {
            panic!("Not a chest");
        };
    }

    #[test]
    fn test_cant_close_closed_chest() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), false).into();
        let action = Action::new(
            0,
            Close {
                dir: Direction::East,
            }
            .into(),
            &world,
        );
        assert!(action.is_err());
    }

    // TODO: Add tests for doors (open/close/can't close wuth items)
}
