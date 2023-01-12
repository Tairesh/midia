use geometry::{Direction, DIR8};
use rand::seq::SliceRandom;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{
            item::{ItemInteract, ItemTag},
            terrain::{Terrain, TerrainInteract, TerrainView},
        },
        Avatar, World,
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
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.is_diggable() {
            return if let Terrain::Pit(..) = tile.terrain {
                No("You can't dig a hole in a hole".to_string())
            } else {
                No(format!("You can't dig {}", tile.terrain.name()))
            };
        }
        if !actor
            .wield
            .iter()
            .any(|i| i.tags().contains(&ItemTag::DigTool))
        {
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
        let pos = action.owner(world).pos + self.dir;
        let items = world.map().get_tile_mut(pos).dig();
        if !items.is_empty() {
            let mut rng = rand::thread_rng();
            let places: Vec<Direction> = DIR8
                .iter()
                .copied()
                .filter(|d| {
                    (pos + *d != action.owner(world).pos)
                        && world.map().get_tile(pos + *d).terrain.is_passable()
                })
                .collect();
            for item in items {
                let delta = places.choose(&mut rng).copied().unwrap();
                world.map().get_tile_mut(pos + delta).items.push(item);
            }
        }
        world.calc_fov();
        world.log().push(LogEvent::new(
            format!("{} dug a hole", action.owner(world).name_for_actions()),
            pos,
            LogCategory::Info,
        ));
    }
}
