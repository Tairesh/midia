use serde::{Deserialize, Serialize};

use crate::game::actions::implements::{Melee, Walk};
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
        let attack = Action::new(unit_id, Melee::new(player).into(), world);
        if let Ok(action) = attack {
            return Some(action);
        }

        Action::new(unit_id, Walk::new(pos.dir_to(player)).into(), world).ok()
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
        if let ActionType::Walk(walk) = action.typ {
            assert_eq!(Direction::West, walk.dir());
        } else {
            panic!("Unexpected monster action: {:?}", action.typ);
        }
    }

    #[test]
    fn test_monster_attack_player() {
        let mut world = prepare_world();
        let npc = add_monster(&mut world, Point::new(1, 0));
        world.plan_test();

        let units = world.units();
        let npc = units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        if let ActionType::Melee(melee) = action.typ {
            assert_eq!(Point::new(0, 0), melee.target());
        } else {
            panic!("Unexpected monster action: {:?}", action.typ);
        }
    }
}
