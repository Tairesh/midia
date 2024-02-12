use serde::{Deserialize, Serialize};

use crate::game::actions::implements::Skip;
use crate::game::{Action, ActionType, World};

use super::super::{AIImpl, AI};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DummyAI;

impl AIImpl for DummyAI {
    fn plan(&mut self, unit_id: usize, world: &World) -> Option<Action> {
        Action::new(unit_id, Skip {}.into(), world).ok()
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::world::tests::{add_dummy, prepare_world};
    use crate::game::Avatar;

    use super::*;

    #[test]
    fn test_dummy_ai() {
        let mut world = prepare_world();
        let npc = add_dummy(&mut world, Point::new(1, 1));
        world.plan_test();

        let units = world.units();
        let npc = units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        assert!(matches!(action.typ, ActionType::Skip(..)));
    }
}
