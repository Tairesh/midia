use super::super::{
    super::{Avatar, World},
    ActionImpl,
    ActionPossibility::{self, Yes},
    ActionType,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Skip {
    length: u32,
}

impl Skip {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(length: u32) -> ActionType {
        Self { length }.into()
    }

    pub fn one() -> ActionType {
        Self::new(1)
    }
}

impl ActionImpl for Skip {
    fn is_possible(&self, _actor_id: usize, _world: &World) -> ActionPossibility {
        Yes(self.length)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar};

    use super::Skip;

    #[test_case(1)]
    #[test_case(2)]
    #[test_case(10)]
    fn test_skipping_time(ticks: u32) {
        let mut world = prepare_world();

        assert_eq!(0, world.meta.current_tick);
        world
            .units_mut()
            .player_mut()
            .set_action(Some(Action::new(0, Skip::new(ticks), &world).unwrap()));
        world.tick();
        assert_eq!(ticks as u128, world.meta.current_tick);
    }
}
