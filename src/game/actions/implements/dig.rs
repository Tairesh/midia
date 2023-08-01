use geometry::{Direction, DIR8};
use rand::seq::SliceRandom;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{Terrain, TerrainInteract, TerrainView},
        Avatar, ItemQuality, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Dig {
    pub dir: Direction,
}

impl ActionImpl for Dig {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.is_diggable() {
            return if let Terrain::Pit(..) = tile.terrain {
                No("You can't dig a hole in a hole".to_string())
            } else {
                No(format!("You can't dig the {}", tile.terrain.name()))
            };
        }
        if !actor.wield.has_quality(&ItemQuality::Dig) {
            return No("You need a shovel to dig!".to_string());
        }

        Yes(1000)
    }

    fn on_start(&self, action: &Action, world: &mut World) {
        let owner = action.owner(world);
        world.log().push(LogEvent::new(
            format!("{} started digging", owner.name_for_actions()),
            owner.pos,
            LogCategory::Info,
        ));
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let owner = action.owner(world);
        let pos = owner.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile_mut(pos);
        let (terrain, items) = tile.terrain.dig_result();
        tile.terrain = terrain;
        if !items.is_empty() {
            let mut rng = rand::thread_rng();
            let places: Vec<Direction> = DIR8
                .iter()
                .copied()
                .filter(|d| (pos + *d != owner.pos) && map.get_tile(pos + *d).terrain.is_passable())
                .collect();
            for item in items {
                let delta = places.choose(&mut rng).copied().unwrap();
                map.get_tile_mut(pos + delta).items.push(item);
            }
        }

        drop(map);
        world.log().push(LogEvent::new(
            format!("{} dug a hole", owner.name_for_actions()),
            pos,
            LogCategory::Info,
        ));
        world.calc_fov();
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::items::helpers::STONE_SHOVEL;
    use crate::game::map::{terrains::Dirt, Terrain};
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Item};

    use super::Dig;

    #[test]
    fn test_digging() {
        let mut world = prepare_world();
        world.player_mut().wield.clear();
        world.map().get_tile_mut(Point::new(1, 0)).terrain = Dirt::default().into();

        let typ = Dig {
            dir: Direction::East,
        };
        assert!(Action::new(0, typ.into(), &world).is_err());

        world.player_mut().wield.wield(Item::new(STONE_SHOVEL));
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
}
