use enum_dispatch::enum_dispatch;

use super::{
    super::{Avatar, World},
    Action, ActionPossibility,
};

#[enum_dispatch]
pub trait ActionImpl {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility;
    fn on_start(&self, _action: &Action, _world: &mut World) {}
    fn on_step(&self, _action: &Action, _world: &mut World) {}
    fn on_finish(&self, _action: &Action, _world: &mut World) {}
}
