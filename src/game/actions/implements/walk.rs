use std::f32::consts::SQRT_2;

use roguemetry::Direction;

use crate::game::ActionType;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{Passage::Passable, TerrainInteract, TerrainView},
        traits::Name,
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Walk {
    dir: Direction,
}

impl Walk {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(dir: Direction) -> ActionType {
        Self { dir }.into()
    }

    #[cfg(test)]
    pub fn dir(&self) -> Direction {
        self.dir
    }
}

impl ActionImpl for Walk {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let units = world.units();
        let actor = units.get_unit(actor_id);
        let pos = actor.pos() + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.is_passable() {
            return No(format!("You can't walk to the {}", tile.terrain.name()));
        }
        let unit_on_tile = tile.units.iter().copied().next();
        if let Some(unit_id) = unit_on_tile {
            return No(format!(
                "{} is on the way",
                world.units().get_unit(unit_id).name_for_actions()
            ));
        }
        Yes({
            let k_diagonal = if self.dir.is_diagonal() { SQRT_2 } else { 1.0 };
            let k_character = actor.char_sheet().walk_koeff();
            let k = k_diagonal * k_character;
            if let Passable(pass_time) = tile.terrain.passage() {
                (pass_time as f32 * k).round() as u32
            } else {
                0
            }
        })
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        world.move_avatar(action.owner, self.dir);
        let pos = world.units().get_unit(action.owner).pos();
        if action.length > 20 && action.owner == 0 {
            world.log().push(LogEvent::new(
                format!(
                    "It takes a long time to walk through the {}",
                    world.map().get_tile(pos).terrain.name()
                ),
                pos,
                LogCategory::Info,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::{Direction, Point};

    use crate::game::actions::implements::Skip;
    use crate::game::map::terrains::{Boulder, BoulderSize, Dirt};
    use crate::game::world::tests::{add_dummy, prepare_world};
    use crate::game::{Action, ActionType, Avatar};

    use super::Walk;

    #[test]
    fn test_walking() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();

        let typ = Walk {
            dir: Direction::East,
        };
        let action = Action::new(0, typ.into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(Point::new(1, 0), world.units().player().pos());
    }

    #[test]
    fn test_walking_fail_to_impassable_terrain() {
        let world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::new(BoulderSize::Huge).into();

        assert!(Action::new(
            0,
            Walk {
                dir: Direction::East,
            }
            .into(),
            &world,
        )
        .is_err());
    }

    #[test]
    fn test_walking_fail_to_unit() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();
        add_dummy(&mut world, Point::new(1, 0));

        assert!(Action::new(
            0,
            Walk {
                dir: Direction::East
            }
            .into(),
            &world,
        )
        .is_err());
    }

    #[test]
    fn test_fail_walking_two_units_to_same_place() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(0, 1)).terrain = Dirt::default().into();
        let npc = add_dummy(&mut world, Point::new(1, 0));

        let action = Action::new(
            0,
            Walk {
                dir: Direction::South,
            }
            .into(),
            &world,
        )
        .unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        let action = Action::new(
            npc,
            Walk {
                dir: Direction::SouthWest,
            }
            .into(),
            &world,
        )
        .unwrap();
        world.units_mut().get_unit_mut(npc).set_action(Some(action));
        world.tick();
        assert_eq!(Point::new(0, 1), world.units().player().pos());
        assert_eq!(Point::new(1, 0), world.units().get_unit(npc).pos());
        assert!(world.units().player().action().is_none());

        let action = Action::new(0, Skip::one().into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(action));
        world.tick();
        assert!(matches!(
            world.units().get_unit(npc).action().unwrap().typ,
            ActionType::Skip(..)
        ));
        assert_eq!(Point::new(1, 0), world.units().get_unit(npc).pos());
        assert_eq!(1, world.map().get_tile(Point::new(0, 1)).units.len());
        assert_eq!(1, world.map().get_tile(Point::new(1, 0)).units.len());
        assert_eq!(0, world.map().get_tile(Point::new(0, 0)).units.len());
    }

    #[test]
    fn test_two_monsters_cant_walk_to_same_tile() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 1)).terrain = Dirt::default().into();
        let npc1 = add_dummy(&mut world, Point::new(1, 0));
        let npc2 = add_dummy(&mut world, Point::new(0, 1));

        let action = Action::new(npc1, Walk::new(Direction::South).into(), &world).unwrap();
        world
            .units_mut()
            .get_unit_mut(npc1)
            .set_action(Some(action));
        let action = Action::new(npc2, Walk::new(Direction::East).into(), &world).unwrap();
        world
            .units_mut()
            .get_unit_mut(npc2)
            .set_action(Some(action));
        let skip = Action::new(0, Skip::new(20).into(), &world).unwrap();
        world.units_mut().player_mut().set_action(Some(skip));
        world.tick();
        let pos1 = world.units().get_unit(npc1).pos();
        let pos2 = world.units().get_unit(npc2).pos();
        assert!(
            (Point::new(1, 1) == pos1 && Point::new(0, 1) == pos2)
                || (Point::new(1, 1) == pos2 && Point::new(1, 0) == pos1)
        );
        assert!(matches!(
            world.units().get_unit(npc1).action().unwrap().typ,
            ActionType::Skip(..)
        ));
        assert!(matches!(
            world.units().get_unit(npc2).action().unwrap().typ,
            ActionType::Skip(..)
        ));
    }

    // TODO: bug with walking when monster already dead
}
