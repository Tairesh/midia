use std::f32::consts::SQRT_2;

use geometry::Direction;

use crate::game::traits::Name;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{Passage::Passable, TerrainInteract, TerrainView},
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Walk {
    pub dir: Direction,
}

impl ActionImpl for Walk {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.is_passable() {
            return No(format!("You can't walk to the {}", tile.terrain.name()));
        }
        let unit_on_tile = tile.units.iter().copied().next();
        if let Some(unit_id) = unit_on_tile {
            let unit = world.units.get_unit(unit_id);
            return No(format!("{} is on the way", unit.name_for_actions()));
        }
        Yes({
            let k_diagonal = if self.dir.is_diagonal() { SQRT_2 } else { 1.0 };
            let k_character = actor.personality.char_sheet.walk_koeff();
            let k = k_diagonal * k_character;
            if let Passable(pass_time) = tile.terrain.passage() {
                f32::round(pass_time * k) as u32
            } else {
                0
            }
        })
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        world.move_avatar(action.owner, self.dir);
        let pos = world.units.get_unit(action.owner).pos;
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
    use geometry::{Direction, Point};

    use crate::game::actions::implements::Skip;
    use crate::game::map::terrains::{Boulder, BoulderSize, Dirt};
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::Action;

    use super::Walk;

    #[test]
    fn test_walking() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();

        let typ = Walk {
            dir: Direction::East,
        };
        world.units.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        world.tick();

        assert_eq!(Point::new(1, 0), world.units.player().pos);
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
        add_npc(&mut world, Point::new(1, 0));

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
        world.map().get_tile_mut(Point::new(1, 1)).terrain = Dirt::default().into();
        let npc = add_npc(&mut world, Point::new(1, 0));

        world.units.player_mut().action = Some(
            Action::new(
                0,
                Walk {
                    dir: Direction::South,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.units.get_unit_mut(npc).action = Some(
            Action::new(
                npc,
                Walk {
                    dir: Direction::SouthWest,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();
        assert_eq!(Point::new(0, 1), world.units.player().pos);
        assert_eq!(Point::new(1, 0), world.units.get_unit(npc).pos);
        assert!(world.units.player().action.is_none());

        world.units.player_mut().action = Some(Action::new(0, Skip {}.into(), &world).unwrap());
        world.tick();
        // do not check npc.action because it can be already new one, selected by AI
        assert_eq!(Point::new(1, 0), world.units.get_unit(npc).pos);
        assert_eq!(1, world.map().get_tile(Point::new(0, 1)).units.len());
        assert_eq!(1, world.map().get_tile(Point::new(1, 0)).units.len());
        assert_eq!(0, world.map().get_tile(Point::new(0, 0)).units.len());
    }
}
