use enum_dispatch::enum_dispatch;

use super::{
    super::{Avatar, World},
    implements::{
        Close, Dig, DropMainHand, Melee, Open, Read, Reload, Shoot, Skip, Throw, Walk, Wear,
        WieldFromGround,
    },
    Action, ActionImpl, ActionPossibility,
};

#[enum_dispatch(ActionImpl)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    Skip,
    Walk,
    WieldFromGround,
    DropMainHand,
    Dig,
    Read,
    Open,
    Close,
    Wear,
    Melee,
    Throw,
    Shoot,
    Reload,
}
