use std::f32::consts::SQRT_2;

use geometry::Direction;

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
            let unit = world.get_unit(unit_id);
            return No(format!("{} is on the way", unit.name_for_actions()));
        }
        Yes({
            let k_diagonal = if self.dir.is_diagonal() { SQRT_2 } else { 1.0 };
            let k_appearance = actor.personality.appearance.walk_koeff();
            let k = k_diagonal * k_appearance;
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
