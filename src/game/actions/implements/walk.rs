use std::f32::consts::SQRT_2;

use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{
            passage::Passage::Passable,
            terrain::{TerrainInteract, TerrainView},
        },
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
            let unit = world.get_unit(unit_id);
            return No(format!("{} is on the way", unit.name_for_actions()));
        }
        Yes({
            let k_diagonal = match self.dir {
                Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest => SQRT_2,
                _ => 1.0,
            };
            let k_age = match actor.personality.appearance.age {
                0 => 100.0,
                1..=3 => 10.0,
                4..=10 => 3.0,
                11.. => 1.0,
            };
            let k = k_diagonal * k_age;
            if let Passable(pass_time) = tile.terrain.passage() {
                f32::round(pass_time * k) as u32
            } else {
                0
            }
        })
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        world.move_avatar(action.owner, self.dir);
        let pos = world.get_unit(action.owner).pos;
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
