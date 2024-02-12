use serde::{Deserialize, Serialize};

use crate::game::actions::implements::Walk;
use crate::game::{Action, ActionType, Avatar, World};

use super::super::{AIImpl, AI};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicMonsterAI;

impl AIImpl for BasicMonsterAI {
    fn plan(&mut self, unit_id: usize, world: &World) -> Option<Action> {
        let units = world.units();
        let unit = units.get_unit(unit_id).as_monster()?;
        let pos = unit.pos();
        let player = units.player().pos();
        let dir = pos.dir_to(player);
        let action = Walk { dir };
        Action::new(unit_id, action.into(), world).ok()
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::actions::implements::Walk;
    use crate::game::world::tests::{add_monster, prepare_world};
    use crate::game::Avatar;

    use super::*;

    #[test]
    fn test_monster_walk_to_player() {
        let mut world = prepare_world();
        let npc = add_monster(&mut world, Point::new(7, 0));
        world.plan_test();

        let units = world.units();
        let npc = units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        assert!(
            matches!(
                action.typ,
                ActionType::Walk(Walk {
                    dir: Direction::West
                })
            ),
            "Unexpected monster action: {:?}",
            action.typ
        );
    }
}
