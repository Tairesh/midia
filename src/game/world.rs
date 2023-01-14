use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use geometry::{Direction, Point, TwoDimDirection};

use crate::savefile::{self, GameView, Meta, SaveError};

use super::{
    map::{field_of_view_set, Fov, TerrainView},
    races::{Appearance, FurColor, Gender, MainHand, Mind, Personality, Race, Sex},
    savage::CharSheet,
    Action, Avatar, Chunk, ChunkPos, Log, Map, TilePos,
};

// TODO: weather and outside lighting system
const VISION_RANGE: i32 = 64;

pub struct World {
    pub meta: Meta,
    pub game_view: GameView,
    units: Vec<Avatar>,
    // TODO: move units to separate struct probably
    loaded_units: HashSet<usize>,
    map: RefCell<Map>,
    fov: Fov,
    log: RefCell<Log>,
    // TODO: add Rng created with seed
    // TODO: add WorldLog
}

impl World {
    pub fn new(
        meta: Meta,
        game_view: GameView,
        log: Log,
        units: Vec<Avatar>,
        chunks: HashMap<ChunkPos, Chunk>,
    ) -> Self {
        let changed = chunks.keys().copied().collect();
        let loaded_units = HashSet::from([0]);
        let mut world = Self {
            map: RefCell::new(Map {
                seed: meta.seed.clone(),
                chunks,
                changed,
            }),
            meta,
            game_view,
            units,
            loaded_units,
            fov: Fov::default(),
            log: RefCell::new(log),
        };
        world.load_units();
        world.calc_fov();
        world
    }

    pub fn create(meta: Meta, avatar: Avatar) -> Self {
        Self::new(
            meta,
            GameView::default(),
            Log::new(),
            vec![avatar],
            HashMap::new(),
        )
    }

    /// Calls one time after world is created
    pub fn init(mut self) -> Self {
        // TODO: don't forget to remove
        self.add_unit(Avatar::new(
            Personality::new(
                false,
                Appearance {
                    race: Race::Gazan,
                    age: 25,
                    fur_color: Some(FurColor::LightBrown),
                    sex: Sex::Male,
                },
                Mind {
                    name: "Dragan".to_string(),
                    gender: Gender::Male,
                    main_hand: MainHand::Right,
                    alive: true,
                },
            ),
            CharSheet::default(Race::Gazan),
            Point::new(0, -5),
        ));
        self.add_unit(Avatar::new(
            Personality::new(
                false,
                Appearance {
                    race: Race::Nyarnik,
                    age: 22,
                    fur_color: None,
                    sex: Sex::Female,
                },
                Mind {
                    name: "Shasha".to_string(),
                    gender: Gender::Female,
                    main_hand: MainHand::Left,
                    alive: true,
                },
            ),
            CharSheet::default(Race::Nyarnik),
            Point::new(-3, -5),
        ));
        self.add_unit(Avatar::new(
            Personality::new(
                false,
                Appearance {
                    race: Race::Gazan,
                    age: 20,
                    fur_color: Some(FurColor::Ginger),
                    sex: Sex::Male,
                },
                Mind {
                    name: "Yasma".to_string(),
                    gender: Gender::Male,
                    main_hand: MainHand::Right,
                    alive: true,
                },
            ),
            CharSheet::default(Race::Gazan),
            Point::new(3, -5),
        ));
        self.add_unit(Avatar::new(
            Personality::new(
                false,
                Appearance {
                    race: Race::Lagnam,
                    age: 25,
                    fur_color: Some(FurColor::White),
                    sex: Sex::Male,
                },
                Mind {
                    name: "Grem".to_string(),
                    gender: Gender::Male,
                    main_hand: MainHand::Right,
                    alive: true,
                },
            ),
            CharSheet::default(Race::Lagnam),
            Point::new(6, -5),
        ));
        self.add_unit(Avatar::new(
            Personality::new(
                false,
                Appearance {
                    race: Race::Totik,
                    age: 20,
                    fur_color: None,
                    sex: Sex::Male,
                },
                Mind {
                    name: "Unnamed totik".to_string(),
                    gender: Gender::Male,
                    main_hand: MainHand::Right,
                    alive: true,
                },
            ),
            CharSheet::default(Race::Totik),
            Point::new(-6, -5),
        ));

        self.units.iter().enumerate().for_each(|(i, unit)| {
            self.map.borrow_mut().get_tile_mut(unit.pos).on_step(i);
        });
        self
    }

    pub fn calc_fov(&mut self) {
        self.fov.set_visible(field_of_view_set(
            self.player().pos,
            VISION_RANGE,
            &self.map.borrow(),
        ));
    }

    // TODO: move this to savefile::save
    fn make_data(&self) -> Result<String, SaveError> {
        let mut data = serde_json::to_string(&self.meta).map_err(SaveError::from)?;
        data.push('\n');
        data.push_str(
            serde_json::to_string(&self.game_view)
                .map_err(SaveError::from)?
                .as_str(),
        );
        data.push('\n');
        data.push_str(
            serde_json::to_string(&self.log)
                .map_err(SaveError::from)?
                .as_str(),
        );
        for unit in &self.units {
            data.push('\n');
            data.push_str(
                serde_json::to_string(unit)
                    .map_err(SaveError::from)?
                    .as_str(),
            );
        }
        data.push_str("\n/units");
        let mut map = self.map();
        for coords in map.changed.clone() {
            let chunk = map.get_chunk(coords);
            data.push('\n');
            data.push_str(
                serde_json::to_string(chunk)
                    .map_err(SaveError::from)?
                    .as_str(),
            );
        }
        data.push_str("\n/chunks");

        Ok(data)
    }

    pub fn save(&mut self) {
        self.meta.update_before_save();
        savefile::save(
            &self.meta.path,
            self.make_data()
                .expect("Error on preparing world data!")
                .as_str(),
        )
        .map_err(|e| panic!("Error on saving world to {:?}: {e:?}", self.meta.path))
        .ok();
    }

    pub fn map(&self) -> RefMut<Map> {
        self.map.borrow_mut()
    }

    pub fn is_visible<P: Into<Point>>(&self, pos: P) -> bool {
        self.fov.visible().contains(&pos.into())
    }

    pub fn get_unit(&self, unit_id: usize) -> &Avatar {
        self.units.get(unit_id).unwrap()
    }

    pub fn get_unit_mut(&mut self, unit_id: usize) -> &mut Avatar {
        self.units.get_mut(unit_id).unwrap()
    }

    pub fn player(&self) -> &Avatar {
        self.get_unit(0)
    }

    pub fn player_mut(&mut self) -> &mut Avatar {
        self.get_unit_mut(0)
    }

    pub fn move_avatar(&mut self, unit_id: usize, dir: Direction) {
        let mut pos = self.units.get(unit_id).unwrap().pos;
        let (old_chunk, _) = pos.to_chunk();
        self.map().get_tile_mut(pos).off_step(unit_id);
        pos += dir;
        if let Some(unit) = self.units.get_mut(unit_id) {
            unit.pos = pos;
            if let Ok(dir) = TwoDimDirection::try_from(dir) {
                unit.vision = dir;
            }
        }
        self.map().get_tile_mut(pos).on_step(unit_id);
        if unit_id == 0 && old_chunk != pos.to_chunk().0 {
            self.load_units();
        }
        if unit_id == 0 {
            self.calc_fov();
        }
    }

    pub fn log(&self) -> RefMut<Log> {
        self.log.borrow_mut()
    }

    // TODO: move this somewhere else
    pub fn this_is(&self, pos: Point, multiline: bool) -> String {
        let mut map = self.map();
        let tile = map.get_tile(pos);
        let mut this_is = format!("This is a {}.", tile.terrain.name());
        if multiline {
            this_is = this_is.replace(". ", ".\n");
        }

        if !tile.items.is_empty() || !tile.units.is_empty() {
            this_is.push(if multiline { '\n' } else { ' ' });
            this_is.push_str("Here you see: ");
            if multiline {
                this_is.push('\n');
            }
        }

        let mut items: Vec<String> = Vec::with_capacity(tile.items.len() + tile.units.len());
        if !tile.items.is_empty() {
            items.append(
                &mut tile
                    .items
                    .iter()
                    .map(|item| (if multiline { " - " } else { "" }).to_string() + item.name())
                    .collect(),
            );
        }
        if !tile.units.is_empty() {
            items.append(
                &mut tile
                    .units
                    .iter()
                    .copied()
                    .map(|i| {
                        let unit = self.units.get(i).unwrap();
                        (if multiline { " - " } else { "" }).to_string()
                            + unit.name_for_actions().as_str()
                    })
                    .collect(),
            );
        }
        this_is += items.join(if multiline { "\n" } else { ", " }).as_str();

        this_is
    }

    /// Doing actions that should be done
    fn act(&mut self) {
        let actions: Vec<Action> = self
            .units
            .iter()
            .rev()
            .filter(|u| u.action.is_some())
            .map(|u| u.action.as_ref().unwrap().clone())
            .collect();
        for action in actions {
            action.act(self);
            if self.meta.current_tick >= action.finish {
                self.get_unit_mut(action.owner).action = None;
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_unit(&mut self, unit: Avatar) -> usize {
        let pos = unit.pos;
        self.units.push(unit);
        self.load_units();
        let new_id = self.units.len() - 1;
        self.map().get_tile_mut(pos).units.insert(new_id);

        new_id
    }

    fn load_units(&mut self) {
        self.loaded_units.clear();
        let center = self.player().pos;
        for (i, unit) in self.units.iter().enumerate() {
            let pos = unit.pos;
            let dist = pos.square_distance(center);
            if dist <= Self::BUBBLE_SQUARE_RADIUS {
                self.loaded_units.insert(i);
            } else {
                self.loaded_units.remove(&i);
            }
        }
    }

    pub const BUBBLE_SQUARE_RADIUS: u32 = 128 * 128;
    pub const SPEND_LIMIT: u32 = 100; // TODO: probably it should be about 10-50

    pub fn tick(&mut self) {
        self.act();

        let mut spend = 0;
        while self.player().action.is_some() && spend < Self::SPEND_LIMIT {
            self.meta.current_tick += 1;
            spend += 1;
            self.act();

            // TODO: npcs AI
            // let mut unit_wants_actions = HashMap::new();
            // for (unit_id, unit) in self.units.iter_mut().skip(1).enumerate() {
            //     if unit.action.is_none() {
            //         if let Some(brain) = &mut unit.ai {
            //             brain.plan();
            //             if let Some(action_type) = brain.action() {
            //                 // +1 is because we skipped first one in enumeration
            //                 unit_wants_actions.insert(unit_id + 1, action_type);
            //             }
            //         }
            //     }
            // }
            // for (unit_id, typ) in unit_wants_actions {
            //     self.units.get_mut(unit_id).unwrap().action = Action::new(unit_id, typ, self).ok();
            // }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use geometry::Point;

    use super::{
        super::{
            actions::implements::{Skip, Walk},
            map::terrains::{Boulder, BoulderSize, Dirt},
            races::{
                tests::personality::{old_queer, tester_girl},
                Race,
            },
        },
        savefile::{GameView, Meta},
        Action, Avatar, CharSheet, Direction, Log, TerrainView, World,
    };

    pub fn prepare_world() -> World {
        let mut player = tester_girl();
        player.is_player = true;
        World::new(
            Meta::new("test", "test"),
            GameView::default(),
            Log::new(),
            vec![Avatar::dressed_default(
                player,
                CharSheet::default(Race::Gazan),
                Point::new(0, 0),
            )],
            HashMap::new(),
        )
    }

    pub fn add_npc(world: &mut World, pos: Point) -> usize {
        world.add_unit(Avatar::new(old_queer(), CharSheet::default(Race::Bug), pos))
    }

    #[test]
    pub fn test_moving_other_unit() {
        let mut world = prepare_world();
        add_npc(&mut world, Point::new(1, 0));

        assert_eq!(2, world.units.len());
        world.map().get_tile_mut(Point::new(2, 0)).terrain = Dirt::default().into();
        let action = Action::new(
            1,
            Walk {
                dir: Direction::East,
            }
            .into(),
            &world,
        )
        .unwrap();
        let length = action.length;
        if let Some(npc) = world.units.get_mut(1) {
            npc.action = Some(action);
        } else {
            unreachable!();
        }
        assert_eq!(Point::new(0, 0), world.player().pos);
        assert_eq!(Point::new(1, 0), world.units.get(1).unwrap().pos);
        for _ in 0..length {
            world.player_mut().action = Some(Action::new(0, Skip {}.into(), &world).unwrap());
            world.tick();
        }
        assert_eq!(Point::new(0, 0), world.player().pos);
        assert_eq!(Point::new(2, 0), world.units.get(1).unwrap().pos)
    }

    #[test]
    pub fn test_fov() {
        let mut world = prepare_world();
        assert!(world.fov.visible().contains(&world.player().pos.into()));

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();
        world.map().get_tile_mut(Point::new(2, 0)).terrain = Boulder::new(BoulderSize::Huge).into();
        assert!(!world
            .map()
            .get_tile(Point::new(2, 0))
            .terrain
            .is_transparent());
        world.map().get_tile_mut(Point::new(3, 0));

        world.move_avatar(0, Direction::East);
        assert!(world.is_visible(Point::new(1, 0)));
        assert!(world.is_visible(Point::new(2, 0)));
        assert!(!world.is_visible(Point::new(3, 0)));
    }
}
