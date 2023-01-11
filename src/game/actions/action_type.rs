use enum_dispatch::enum_dispatch;

use super::{
    super::{Avatar, World},
    implements::{Dig, Drop, Read, Skip, Walk, Wield},
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
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point, DIR8};

    use super::{
        super::super::{
            bodies::{Freshness, OrganData},
            map::{
                items::{Axe, BodyPart, BodyPartType, Gravestone, Shovel},
                terrains::{Boulder, BoulderSize, Dirt, Grave, GraveData, GraveVariant},
                Item, Terrain,
            },
            races::{tests::personality::dead_boy, Gender, MainHand, SkinTone},
            world::tests::{add_zombie, prepare_world},
        },
        Action, Dig, Drop, Read, Skip, Walk, Wield,
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
        add_zombie(&mut world, Point::new(1, 0));

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
        let zombie = add_zombie(&mut world, Point::new(1, 0));

        world.player_mut().action = Some(
            Action::new(
                0,
                Walk {
                    dir: Direction::SouthEast,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.get_unit_mut(zombie).action = Some(
            Action::new(
                zombie,
                Walk {
                    dir: Direction::South,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();
        assert_eq!(Point::new(1, 1), world.player().pos);
        assert_eq!(Point::new(1, 0), world.get_unit(zombie).pos);
        assert!(world.player().action.is_none());

        world.player_mut().action = Some(Action::new(0, Skip {}.into(), &world).unwrap());
        world.tick();
        // do not check zombie.action because it can be already new one, selected by AI
        assert_eq!(Point::new(1, 0), world.get_unit(zombie).pos);
        assert_eq!(1, world.map().get_tile(Point::new(1, 1)).units.len());
        assert_eq!(1, world.map().get_tile(Point::new(1, 0)).units.len());
        assert_eq!(0, world.map().get_tile(Point::new(0, 0)).units.len());
    }

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(Axe::new().into());

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
        assert!(matches!(item, Item::Axe(..)));
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
        world.player_mut().wield.push(Axe::new().into());

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
        assert!(matches!(item, Item::Axe(..)));
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

        world.player_mut().wield.push(Shovel::new().into());
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }

        assert_eq!(Point::new(0, 0), world.player().pos);
        assert!(matches!(
            world.map().get_tile(Point::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));

        let character = dead_boy();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Grave::new(
            GraveVariant::New,
            GraveData {
                character,
                death_year: 255,
            },
        )
        .into();
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }
        assert!(matches!(
            world.map().get_tile(Point::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));
        let mut corpse = None;
        let mut gravestone = None;
        for dir in DIR8 {
            for item in world
                .map()
                .get_tile_mut(Point::new(1, 0) + dir)
                .items
                .iter()
            {
                match item {
                    Item::Corpse(..) => {
                        corpse = Some(item.clone());
                    }
                    Item::Gravestone(..) => {
                        gravestone = Some(item.clone());
                    }
                    _ => {}
                }
            }
        }
        assert!(corpse.is_some());
        if let Some(corpse) = corpse {
            if let Item::Corpse(corpse) = corpse {
                let ch = &corpse.character;
                let body = &corpse.body;
                assert_eq!("Dead Boy", ch.mind.name);
                assert_eq!(SkinTone::Almond, ch.appearance.skin_tone);
                assert_eq!(Gender::Male, ch.mind.gender);
                assert_eq!(9, ch.appearance.age);
                assert_eq!(MainHand::Right, ch.mind.main_hand);
                assert!(matches!(
                    body.parts.get(&Point::new(0, 0)),
                    Some(BodyPart {
                        typ: BodyPartType::Torso,
                        data: OrganData {
                            freshness: Freshness::Rotten,
                            ..
                        },
                        ..
                    })
                ));
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        assert!(gravestone.is_some());
        if let Some(gravestone) = gravestone {
            if let Item::Gravestone(gravestone) = gravestone {
                let data = &gravestone.data;
                assert_eq!("Dead Boy", data.character.mind.name);
                assert_eq!(SkinTone::Almond, data.character.appearance.skin_tone);
                assert_eq!(Gender::Male, data.character.mind.gender);
                assert_eq!(9, data.character.appearance.age);
                assert_eq!(MainHand::Right, data.character.mind.main_hand);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_reading() {
        let mut world = prepare_world();

        let character = dead_boy();
        let data = GraveData {
            character,
            death_year: 255,
        };
        world.map().get_tile_mut(Point::new(1, 0)).terrain =
            Grave::new(GraveVariant::New, data.clone()).into();
        world.player_mut().action = Some(
            Action::new(
                0,
                Read {
                    dir: Direction::East,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        while world.player().action.is_some() {
            world.tick();
            for event in world.log().new_events() {
                assert_eq!("You read on gravestone: Dead Boy. 246 — 255", event.msg);
            }
        }

        world.map().get_tile_mut(Point::new(0, 1)).terrain = Dirt::default().into();
        world.map().get_tile_mut(Point::new(0, 1)).items.clear();
        let typ = Read {
            dir: Direction::South,
        };
        assert!(Action::new(0, typ.into(), &world).is_err());

        world
            .map()
            .get_tile_mut(Point::new(0, 1))
            .items
            .push(Gravestone::new(data).into());

        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
            for event in world.log().new_events() {
                assert_eq!("You read on gravestone: Dead Boy. 246 — 255", event.msg);
            }
        }
    }
}
