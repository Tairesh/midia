use enum_dispatch::enum_dispatch;

use super::{
    super::{Avatar, World},
    implements::{Close, Dig, Drop, Open, Read, Skip, Walk, Wield},
    Action, ActionImpl, ActionPossibility,
};

#[enum_dispatch(ActionImpl)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    Skip,
    Walk,
    Wield,
    Drop,
    Dig,
    Read,
    Open,
    Close,
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use super::{
        super::super::{
            map::{
                items::helpers::{axe, random_book, shovel},
                terrains::{Boulder, BoulderSize, Chest, Dirt},
                Terrain, TerrainInteract,
            },
            world::tests::{add_npc, prepare_world},
        },
        Action, Close, Dig, Drop, Open, Read, Skip, Walk, Wield,
    };

    #[test]
    fn test_walking() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();

        let typ = Walk {
            dir: Direction::East,
        };
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        world.tick();

        assert_eq!(Point::new(1, 0), world.player().pos);
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

        world.player_mut().action = Some(
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
        world.get_unit_mut(npc).action = Some(
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
        assert_eq!(Point::new(0, 1), world.player().pos);
        assert_eq!(Point::new(1, 0), world.get_unit(npc).pos);
        assert!(world.player().action.is_none());

        world.player_mut().action = Some(Action::new(0, Skip {}.into(), &world).unwrap());
        world.tick();
        // do not check npc.action because it can be already new one, selected by AI
        assert_eq!(Point::new(1, 0), world.get_unit(npc).pos);
        assert_eq!(1, world.map().get_tile(Point::new(0, 1)).units.len());
        assert_eq!(1, world.map().get_tile(Point::new(1, 0)).units.len());
        assert_eq!(0, world.map().get_tile(Point::new(0, 0)).units.len());
    }

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world.map().get_tile_mut(Point::new(1, 0)).items.push(axe());

        assert!(world.player().wield.is_empty());
        assert_eq!(0, world.meta.current_tick);

        world.player_mut().action = Some(
            Action::new(
                0,
                Wield {
                    dir: Direction::East,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();

        assert_eq!(Point::new(0, 0), world.player().pos);
        assert_eq!(1, world.player().wield.len());
        let item = world.player().wield.first().unwrap();
        assert_eq!(item.proto.id, axe().proto.id);
    }

    #[test]
    fn test_skipping_time() {
        let mut world = prepare_world();

        assert_eq!(0, world.meta.current_tick);
        world.player_mut().action = Some(Action::new(0, Skip {}.into(), &world).unwrap());
        world.tick();
        assert_eq!(1, world.meta.current_tick);
    }

    #[test]
    fn test_dropping() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(0, 0)).terrain = Dirt::default().into();
        world.map().get_tile_mut(Point::new(0, 0)).items.clear();
        world.player_mut().wield.clear();
        world.player_mut().wield.push(axe());

        world.player_mut().action = Some(
            Action::new(
                0,
                Drop {
                    item_id: 0,
                    dir: Direction::Here,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();

        assert_eq!(Point::new(0, 0), world.player().pos);
        assert_eq!(0, world.player().wield.len());
        let mut map = world.map();
        assert_eq!(1, map.get_tile(Point::new(0, 0)).items.len());
        let item = map.get_tile(Point::new(0, 0)).items.first().unwrap();
        assert_eq!(item.proto.id, axe().proto.id);
    }

    #[test]
    fn test_digging() {
        let mut world = prepare_world();
        world.player_mut().wield.clear();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();

        let typ = Dig {
            dir: Direction::East,
        };
        assert!(Action::new(0, typ.into(), &world).is_err());

        world.player_mut().wield.push(shovel());
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }

        assert_eq!(Point::new(0, 0), world.player().pos);
        assert!(matches!(
            world.map().get_tile(Point::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));
    }

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

    #[test]
    fn test_opening() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), false).into();

        let typ = Open {
            dir: Direction::East,
        };
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.player_mut().action = Some(action);
            while world.player().action.is_some() {
                world.tick();
            }
        } else {
            panic!("Cannot open");
        }

        assert!(world.log().new_events()[0].msg.contains("opened"));
        assert!(world
            .map()
            .get_tile(Point::new(1, 0))
            .terrain
            .can_be_closed());
    }

    #[test]
    fn test_closing() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Chest::new(Vec::new(), true).into();

        let typ = Close {
            dir: Direction::East,
        };
        if let Ok(action) = Action::new(0, typ.into(), &world) {
            world.player_mut().action = Some(action);
            while world.player().action.is_some() {
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
