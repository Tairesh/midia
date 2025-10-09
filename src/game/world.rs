use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use roguemetry::{Direction, Point};

use crate::game::map::items::helpers::BONE_KNIFE;

use super::{
    super::{
        lang,
        savefile::{self, GameView, Meta},
        settings::Settings,
    },
    ai::{AIImpl, AIManager, AI},
    map::{field_of_view_set, Fov, TerrainView},
    races::{BodyColor, Pronouns, Race, Sex},
    savage::HitResult,
    traits::Name,
    units::{Appearance, Avatar, Monster, Player, Units},
    Action, CharSheet, Chunk, ChunkPos, Item, Log, LogEvent, Map, TilePos,
};

pub struct World {
    pub meta: Meta,
    pub game_view: GameView,
    pub units: Units,
    pub map: Map,
    fov: Fov,
    pub log: RefCell<Log>,
    // TODO: add Rng created with seed
    // TODO: add WorldLog
}

impl World {
    pub fn new(
        meta: Meta,
        game_view: GameView,
        log: Log,
        units: HashMap<usize, Box<dyn Avatar>>,
        chunks: HashMap<ChunkPos, Chunk>,
    ) -> Self {
        let changed = chunks.keys().copied().collect();
        let mut world = Self {
            map: Map::new(meta.seed, chunks, changed),
            meta,
            game_view,
            units: Units::new(units),
            fov: Fov::default(),
            log: RefCell::new(log),
        };
        world.units.load_units();
        world.calc_fov();
        world
    }

    pub fn create(meta: Meta, avatar: Player) -> Self {
        let units = HashMap::from([(0, Box::new(avatar) as Box<dyn Avatar>)]);
        let mut world = Self::new(meta, GameView::default(), Log::new(), units, HashMap::new());

        // TODO: don't forget to remove
        world
            .map
            .get_tile_mut(Point::new(0, 0))
            .items
            .push(Item::new(BONE_KNIFE));
        world.add_unit(Box::new(Monster::new(
            AI::BasicMonster,
            Point::new(0, 5),
            "green bug".to_string(),
            Appearance {
                race: Race::Bug,
                age: 1,
                body_color: Some(BodyColor::Lime),
                sex: Sex::Other,
            },
            Pronouns::ItIts,
            CharSheet::default(false, Race::Bug),
        )));
        world.add_unit(Box::new(Monster::new(
            AI::BasicMonster,
            Point::new(0, 7),
            "mutant bug queen".to_string(),
            Appearance {
                race: Race::Bug,
                age: 2,
                body_color: Some(BodyColor::Red),
                sex: Sex::Female,
            },
            Pronouns::SheHer,
            CharSheet::default(false, Race::Bug),
        )));

        world.units.iter().for_each(|(&i, unit)| {
            world.map.get_tile_mut(unit.pos()).on_step(i);
        });

        world
    }

    pub fn calc_fov(&mut self) {
        let center = self.units.player().pos();
        // TODO: weather and outside lighting system
        // TODO: add light sources
        // TODO: add periodic Notice roll
        // TODO: add memory
        let vision_range = self.units.player().char_sheet().sight_range();
        self.fov
            .set_visible(field_of_view_set(center, vision_range as i32, &self.map));
    }

    pub fn save(&mut self) {
        self.meta.update_before_save();
        savefile::save(self)
            .map_err(|e| {
                panic!(
                    "Error on saving world to {}: {e:?}",
                    self.meta.path.display()
                )
            })
            .ok();
    }

    pub fn is_visible(&self, pos: impl Into<Point>) -> bool {
        self.fov.visible().contains(&pos.into())
    }

    pub fn move_avatar(&mut self, unit_id: usize, dir: Direction) {
        let mut pos = self.units.get_unit(unit_id).pos();
        let (old_chunk, _) = pos.to_chunk();
        self.map.get_tile_mut(pos).off_step(unit_id);
        pos += dir;
        let unit = self.units.get_unit_mut(unit_id);
        unit.set_pos(pos);
        unit.view_mut().try_set_direction(dir);
        let unit = self.units.get_unit(unit_id);
        self.map.get_tile_mut(pos).on_step(unit_id);
        if unit.is_player() && old_chunk != pos.to_chunk().0 {
            self.units.load_units();
        }
        if self.units.get_unit(unit_id).is_player() {
            self.calc_fov();
            self.log().push(LogEvent::debug(
                format!("You moved to {pos:?}"),
                self.units.player().pos(),
            ));
        }
    }

    pub fn log(&self) -> RefMut<'_, Log> {
        self.log.borrow_mut()
    }

    // TODO: move this somewhere else
    pub fn this_is(&self, pos: Point, multiline: bool) -> String {
        let Some(tile) = self.map.get_tile_opt(pos) else {
            return "There is nothing here.".to_string();
        };
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
                            + self.units.get_unit(i).name()
                    })
                    .collect(),
            );
        }
        this_is += items.join(if multiline { "\n" } else { ", " }).as_str();

        this_is
    }

    pub fn add_unit(&mut self, unit: Box<dyn Avatar>) -> usize {
        let pos = unit.pos();
        let new_id = self.units.add_unit(unit);
        self.map.get_tile_mut(pos).units.insert(new_id);

        new_id
    }

    fn plan(&mut self) {
        let mut units_to_act = HashMap::new();
        for unit in self.units.loaded_units() {
            if unit.action().is_some() {
                continue;
            }
            if let Some(ai) = unit.ai() {
                units_to_act.insert(unit.id(), ai);
            }
        }

        for (unit_id, ai) in units_to_act {
            let action = AIManager::instance().plan(ai, unit_id, self);
            self.units.get_unit_mut(unit_id).set_action(action);
        }
    }

    #[cfg(test)]
    pub fn plan_test(&mut self) {
        self.plan();
    }

    /// Doing actions that should be done
    fn act(&mut self) {
        self.shock_out();
        self.plan();

        let actions: Vec<Action> = self
            .units
            .loaded_units()
            .filter_map(|u| u.action().cloned())
            .collect();
        for action in actions {
            if self.units.get_unit(action.owner).char_sheet().is_dead() {
                continue;
            }
            action.act(self);
            if self.meta.current_tick >= action.finish {
                self.units.get_unit_mut(action.owner).set_action(None);
            }
        }
    }

    pub const SPEND_LIMIT: u32 = 100; // TODO: probably it should be about 10-50

    // TODO: move this to AI, probably
    /// Shocked units trying to get out of the shock
    fn shock_out(&mut self) {
        let current_tick = self.meta.current_tick;
        let out_of_shock: Vec<usize> = self
            .units
            .loaded_units()
            .filter(|u| u.char_sheet().can_try_to_shock_out(current_tick))
            .map(Avatar::id)
            .collect();
        for unit_id in out_of_shock {
            if self
                .units
                .get_unit_mut(unit_id)
                .char_sheet_mut()
                .try_to_shock_out(current_tick)
            {
                let unit = self.units.get_unit(unit_id);
                self.log().push(LogEvent::info(
                    format!(
                        "{} {} out of the shock!",
                        unit.name_for_actions(),
                        unit.pronouns().is_are()
                    ),
                    unit.pos(),
                ));
            }
        }
    }

    pub fn apply_damage(&mut self, unit_id: usize, hit: HitResult) {
        if unit_id == 0 && Settings::instance().debug.god_mode {
            return;
        }
        let current_tick = self.meta.current_tick;
        let pos = self.units.get_unit(unit_id).pos();
        let items_dropped = self
            .units
            .get_unit_mut(unit_id)
            .apply_hit(hit, current_tick);
        if let Some(items_dropped) = items_dropped {
            for item in items_dropped {
                self.map.get_tile_mut(pos).items.push(item);
            }
        }

        let unit = self.units.get_unit(unit_id);
        if unit.char_sheet().is_dead() {
            self.log().push(LogEvent::danger(
                format!(
                    "{} {} dead!",
                    unit.name_for_actions(),
                    unit.pronouns().is_are(),
                ),
                pos,
            ));
            self.map.get_tile_mut(pos).units.remove(&unit_id);
            self.units.unload_unit(unit_id);
        }
    }

    pub fn tick(&mut self) {
        self.act();

        let mut spend = 0;
        while self.units.player().action().is_some() && spend < Self::SPEND_LIMIT {
            self.meta.current_tick += 1;
            spend += 1;
            self.act();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use super::{
        super::{
            actions::implements::{Skip, Walk},
            ai::AI,
            map::terrains::{Boulder, BoulderSize, Dirt},
            races::{Pronouns, Sex},
            units::{tests::helpers::tester_girl, Appearance, Avatar, Monster},
            CharSheet, Race,
        },
        savefile::{GameView, Meta},
        Action, Direction, Log, Player, TerrainView, World,
    };
    use crate::game::map::terrains::DirtVariant;
    use crate::game::{AttrLevel, SkillLevel, Terrain};
    use roguemetry::Point;

    pub fn boulder() -> Terrain {
        Boulder::new(BoulderSize::Huge).into()
    }

    pub fn dirt() -> Terrain {
        Dirt::new(DirtVariant::Dirt1).into()
    }

    pub fn prepare_world() -> World {
        let mut world = World::new(
            Meta::new("test", 1),
            GameView::default(),
            Log::new(),
            HashMap::from([(
                0usize,
                Box::new(Player::new(tester_girl(), Point::new(0, 0))) as Box<dyn Avatar>,
            )]),
            HashMap::new(),
        );
        world.map.get_tile_mut(Point::new(0, 0)).terrain = dirt();
        world.map.get_tile_mut(Point::new(0, 0)).units.insert(0);

        world
    }

    pub fn add_dummy(world: &mut World, pos: Point) -> usize {
        world.add_unit(Box::new(Monster::new(
            AI::Dummy,
            pos,
            "Dummy".to_string(),
            Appearance {
                race: Race::Gazan,
                age: 20,
                body_color: None,
                sex: Sex::Other,
            },
            Pronouns::ItIts,
            CharSheet::default(false, Race::Gazan),
        )))
    }

    pub fn add_monster(world: &mut World, pos: Point) -> usize {
        let mut charsheet = CharSheet::default(false, Race::Bug);
        charsheet.attributes.agility = AttrLevel::D6;
        charsheet.attributes.strength = AttrLevel::D6;
        charsheet.attributes.vigor = AttrLevel::D8;
        charsheet.skills.fighting = SkillLevel::D6;
        charsheet.skills.shooting = SkillLevel::D8;
        charsheet.skills.notice = SkillLevel::D6;
        world.add_unit(Box::new(Monster::new(
            AI::BasicMonster,
            pos,
            "Old Bugger".to_string(),
            Appearance {
                race: Race::Bug,
                age: 99,
                body_color: None,
                sex: Sex::Other,
            },
            Pronouns::ItIts,
            charsheet,
        )))
    }

    #[test]
    pub fn test_moving_other_unit() {
        let mut world = prepare_world();
        let monster_id = add_dummy(&mut world, Point::new(1, 0));

        world.map.get_tile_mut(Point::new(2, 0)).terrain = dirt();
        let action = Action::new(1, Walk::new(Direction::East).into(), &world).unwrap();
        let length = action.length;
        let monster = world.units.get_unit_mut(monster_id);
        monster.set_action(Some(action));
        assert_eq!(Point::new(0, 0), world.units.player().pos());
        assert_eq!(Point::new(1, 0), world.units.get_unit(monster_id).pos());
        let action = Action::new(0, Skip::new(length).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();
        assert_eq!(Point::new(0, 0), world.units.player().pos());
        assert_eq!(Point::new(2, 0), world.units.get_unit(monster_id).pos())
    }

    #[test]
    pub fn test_fov() {
        let mut world = prepare_world();
        assert!(world
            .fov
            .visible()
            .contains(&world.units.player().pos().into()));

        world.map.get_tile_mut(Point::new(1, 0)).terrain = dirt();
        world.map.get_tile_mut(Point::new(2, 0)).terrain = boulder();
        assert!(!world
            .map
            .get_tile(Point::new(2, 0))
            .terrain
            .is_transparent());
        world.map.get_tile_mut(Point::new(3, 0));

        world.move_avatar(0, Direction::East);
        assert!(world.is_visible(Point::new(1, 0)));
        assert!(world.is_visible(Point::new(2, 0)));
        assert!(!world.is_visible(Point::new(3, 0)));
    }
}
