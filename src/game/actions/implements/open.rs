use geometry::Direction;

use crate::game::log::{LogCategory, LogEvent};
use crate::game::map::{TerrainInteract, TerrainInteractAction, TerrainView};
use crate::game::traits::Name;
use crate::game::{Action, ActionType, Avatar, World};

use super::super::{
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Open {
    pub dir: Direction,
}

impl Open {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(dir: Direction) -> ActionType {
        Self { dir }.into()
    }
}

impl ActionImpl for Open {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let units = world.units();
        let actor = units.get_unit(actor_id);
        let pos = actor.pos() + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);

        if !tile.terrain.supports_action(TerrainInteractAction::Open) {
            return No(format!("You can't open the {}", tile.terrain.name()));
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
                "{} open{} the {}",
                owner.name_for_actions(),
                if owner.pronouns().verb_ends_with_s() {
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

    use crate::game::map::items::helpers::WOODEN_SPLINTER;
    use crate::game::map::terrains::Chest;
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar, Item, TerrainInteract, TerrainInteractAction};

    use super::Open;

    #[test]
    fn test_opening() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain =
            Chest::new(vec![Item::new(WOODEN_SPLINTER)], false).into();

        let action = Action::new(0, Open::new(Direction::East), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        while world.units().player().action().is_some() {
            world.tick();
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
            .supports_action(TerrainInteractAction::Close));
        assert_eq!(world.map().get_tile(Point::new(1, 0)).items.len(), 1);
        assert_eq!(
            world.map().get_tile(Point::new(1, 0)).items[0].proto().id,
            WOODEN_SPLINTER
        );
    }

    #[test]
    fn test_cant_open_already_opened() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), true).into();

        let action = Action::new(0, Open::new(Direction::East), &world);
        assert!(action.is_err());
    }
}
