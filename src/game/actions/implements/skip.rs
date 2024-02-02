use super::super::{
    super::{Avatar, World},
    ActionImpl,
    ActionPossibility::{self, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Skip {}

impl ActionImpl for Skip {
    fn is_possible(&self, _actor: &Avatar, _world: &World) -> ActionPossibility {
        Yes(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::world::tests::prepare_world;
    use crate::game::Action;

    use super::Skip;

    #[test]
    fn test_skipping_time() {
        let mut world = prepare_world();

        assert_eq!(0, world.meta.current_tick);
        world.units_mut().player_mut().action =
            Some(Action::new(0, Skip {}.into(), &world).unwrap());
        world.tick();
        assert_eq!(1, world.meta.current_tick);
    }
}
