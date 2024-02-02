use std::cell::{Ref, RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::rc::Rc;

use geometry::{Direction, Point, TwoDimDirection};

use super::{
    super::{
        lang,
        savefile::{self, GameView, Meta, SaveError},
    },
    game_data::pregen,
    map::{field_of_view_set, Fov, TerrainView},
    races::{BodyColor, Gender, Race, Sex},
    savage::HitResult,
    traits::Name,
    units::Units,
    units::{Appearance, Mind, Personality},
    Action, Avatar, CharSheet, Chunk, ChunkPos, Log, LogEvent, Map, TilePos,
};

// TODO: weather and outside lighting system
const VISION_RANGE: i32 = 64;

pub struct World {
    pub meta: Meta,
    pub game_view: GameView,
    units: Rc<RefCell<Units>>,
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
        units: HashMap<usize, Avatar>,
        chunks: HashMap<ChunkPos, Chunk>,
    ) -> Self {
        let changed = chunks.keys().copied().collect();
        let mut world = Self {
            map: RefCell::new(Map {
                seed: meta.seed.clone(),
                chunks,
                changed,
            }),
            meta,
            game_view,
            units: Rc::new(RefCell::new(Units::new(units))),
            fov: Fov::default(),
            log: RefCell::new(log),
        };
        world.units_mut().load_units();
        world.calc_fov();
        world
    }

    pub fn create(meta: Meta, avatar: Avatar) -> Self {
        Self::new(
            meta,
            GameView::default(),
            Log::new(),
            HashMap::from([(0, avatar)]),
            HashMap::new(),
        )
    }

    /// Calls one time after world is created
    pub fn init(mut self) -> Self {
        // TODO: don't forget to remove
        self.add_unit(Avatar::new(pregen::npc::dragan(), Point::new(0, -5)));
        self.add_unit(Avatar::new(pregen::npc::shasha(), Point::new(-3, -5)));
        self.add_unit(Avatar::new(pregen::npc::yasma(), Point::new(3, -5)));
        self.add_unit(Avatar::new(pregen::npc::grem(), Point::new(6, -5)));
        self.add_unit(Avatar::new(
            Personality::new(
                Appearance {
                    race: Race::Totik,
                    age: 29,
                    body_color: Some(BodyColor::GreenBlue),
                    sex: Sex::Male,
                },
                Mind {
                    name: "Suh".to_string(),
                    gender: Gender::Male,
                },
                CharSheet::default(true, Race::Totik, 29),
            ),
            Point::new(-6, -5),
        ));
        self.add_unit(Avatar::new(
            Personality::new(
                Appearance {
                    race: Race::Bug,
                    age: 99,
                    body_color: Some(BodyColor::Lime),
                    sex: Sex::Undefined,
                },
                Mind {
                    name: "bug".to_string(),
                    gender: Gender::Custom("bug".to_string()),
                },
                CharSheet::default(false, Race::Bug, 99),
            ),
            Point::new(0, 5),
        ));
        self.add_unit(Avatar::new(
            Personality::new(
                Appearance {
                    race: Race::Bug,
                    age: 12,
                    body_color: Some(BodyColor::Red),
                    sex: Sex::Female,
                },
                Mind {
                    name: "bug queen".to_string(),
                    gender: Gender::Custom("queen".to_string()),
                },
                CharSheet::default(false, Race::Bug, 12),
            ),
            Point::new(2, 5),
        ));

        self.units().iter().for_each(|(&i, unit)| {
            self.map.borrow_mut().get_tile_mut(unit.pos).on_step(i);
        });
        self
    }

    pub fn calc_fov(&mut self) {
        let center = self.units().player().pos;
        self.fov
            .set_visible(field_of_view_set(center, VISION_RANGE, &self.map.borrow()));
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
        for (_, unit) in self.units().iter() {
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

    pub fn is_visible(&self, pos: impl Into<Point>) -> bool {
        self.fov.visible().contains(&pos.into())
    }

    pub fn units_clone(&self) -> Rc<RefCell<Units>> {
        self.units.clone()
    }

    pub fn units_mut(&self) -> RefMut<Units> {
        self.units.borrow_mut()
    }

    pub fn units(&self) -> Ref<Units> {
        self.units.borrow()
    }

    pub fn is_observable(&self, pos: impl Into<Point>) -> bool {
        self.units().player().pos.square_distance(pos.into())
            <= (VISION_RANGE * VISION_RANGE) as u32
    }

    pub fn move_avatar(&mut self, unit_id: usize, dir: Direction) {
        let mut pos = self.units().get_unit(unit_id).pos;
        let (old_chunk, _) = pos.to_chunk();
        self.map().get_tile_mut(pos).off_step(unit_id);
        pos += dir;
        let mut units = self.units_mut();
        let unit = units.get_unit_mut(unit_id);
        unit.pos = pos;
        if let Ok(dir) = TwoDimDirection::try_from(dir) {
            unit.vision = dir;
        }
        self.map().get_tile_mut(pos).on_step(unit_id);
        if unit_id == 0 && old_chunk != pos.to_chunk().0 {
            units.load_units();
        }
        if unit_id == 0 {
            drop(units);
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
        let mut this_is = format!("This is {}.", lang::a(tile.terrain.name()));
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
                        (if multiline { " - " } else { "" }).to_string()
                            + self.units().get_unit(i).name_for_actions().as_str()
                    })
                    .collect(),
            );
        }
        this_is += items.join(if multiline { "\n" } else { ", " }).as_str();

        this_is
    }

    pub fn add_unit(&mut self, unit: Avatar) -> usize {
        let pos = unit.pos;
        let new_id = self.units_mut().add_unit(unit);
        self.map().get_tile_mut(pos).units.insert(new_id);

        new_id
    }

    /// Doing actions that should be done
    fn act(&mut self) {
        let actions: Vec<Action> = self
            .units
            .borrow()
            .iter()
            .filter(|(_, u)| u.action.is_some())
            .map(|(_, u)| u.action.as_ref().unwrap().clone())
            .collect();
        for action in actions {
            action.act(self);
            if self.meta.current_tick >= action.finish {
                self.units_mut().get_unit_mut(action.owner).action = None;
            }
        }
    }

    pub const SPEND_LIMIT: u32 = 100; // TODO: probably it should be about 10-50

    // TODO: move this to AI, probably
    /// Shocked units trying to get out of the shock
    fn shock_out(&mut self) {
        let current_tick = self.meta.current_tick;
        let mut units_to_shock_out = Vec::new();
        for unit in self.units().loaded_units() {
            if unit
                .personality
                .char_sheet
                .can_try_to_shock_out(current_tick)
            {
                units_to_shock_out.push(unit.id);
            }
        }
        let mut units = self.units_mut();
        for unit_id in units_to_shock_out {
            let unit = units.get_unit_mut(unit_id);
            if unit.personality.char_sheet.try_to_shock_out(current_tick) {
                self.log().push(LogEvent::info(
                    format!("{} is out of the shock!", unit.name_for_actions()),
                    unit.pos,
                ));
            }
        }
    }

    pub fn apply_damage(&mut self, unit_id: usize, hit: HitResult) {
        let current_tick = self.meta.current_tick;
        let pos = self.units().get_unit(unit_id).pos;
        let items_dropped = self
            .units
            .borrow_mut()
            .get_unit_mut(unit_id)
            .apply_hit(hit, current_tick);
        for item in items_dropped {
            self.map().get_tile_mut(pos).items.push(item);
        }

        if self.units().get_unit(unit_id).is_dead() {
            self.log().push(LogEvent::danger(
                format!(
                    "{} is dead!",
                    self.units().get_unit(unit_id).name_for_actions()
                ),
                pos,
            ));
            self.map().get_tile_mut(pos).units.remove(&unit_id);
            self.units_mut().unload_unit(unit_id);
        }
    }

    pub fn tick(&mut self) {
        // TODO: test some zero-ticks actions
        self.act();

        let mut spend = 0;
        while self.units().player().action.is_some() && spend < Self::SPEND_LIMIT {
            self.meta.current_tick += 1;
            spend += 1;
            self.act();

            self.shock_out();

            // TODO: npcs AI for loaded units
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
            units::tests::helpers::{old_bugger, tester_girl},
        },
        savefile::{GameView, Meta},
        Action, Avatar, Direction, Log, TerrainView, World,
    };

    pub fn prepare_world() -> World {
        World::new(
            Meta::new("test", "test"),
            GameView::default(),
            Log::new(),
            HashMap::from([(0, Avatar::dressed_default(tester_girl(), Point::new(0, 0)))]),
            HashMap::new(),
        )
    }

    pub fn add_npc(world: &mut World, pos: Point) -> usize {
        world.add_unit(Avatar::new(old_bugger(), pos))
    }

    #[test]
    pub fn test_moving_other_unit() {
        let mut world = prepare_world();
        add_npc(&mut world, Point::new(1, 0));

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
        let mut units = world.units_mut();
        let npc = units.get_unit_mut(1);
        npc.action = Some(action);
        assert_eq!(Point::new(0, 0), units.player().pos);
        assert_eq!(Point::new(1, 0), units.get_unit(1).pos);
        drop(units);
        for _ in 0..length {
            world.units_mut().player_mut().action =
                Some(Action::new(0, Skip {}.into(), &world).unwrap());
            world.tick();
        }
        let units = world.units();
        assert_eq!(Point::new(0, 0), units.player().pos);
        assert_eq!(Point::new(2, 0), units.get_unit(1).pos)
    }

    #[test]
    pub fn test_fov() {
        let mut world = prepare_world();
        assert!(world
            .fov
            .visible()
            .contains(&world.units().player().pos.into()));

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
