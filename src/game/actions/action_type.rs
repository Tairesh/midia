use enum_dispatch::enum_dispatch;

use crate::game::{Action, World};

use super::{
    implements::{
        Close, DropMainHand, Melee, Open, Read, Reload, Shoot, Skip, Throw, Walk, Wear,
        WieldFromGround,
    },
    ActionPossibility,
};

#[enum_dispatch]
pub trait ActionImpl {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility;
    fn on_start(&self, _action: &Action, _world: &mut World) {}
    fn on_step(&self, _action: &Action, _world: &mut World) {}
    fn on_finish(&self, _action: &Action, _world: &mut World) {}
}

#[enum_dispatch(ActionImpl)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    Skip,
    Walk,
    WieldFromGround,
    DropMainHand,
    Read,
    Open,
    Close,
    Wear,
    Melee,
    Throw,
    Shoot,
    Reload,
}
