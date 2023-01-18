use enum_dispatch::enum_dispatch;

use super::{
    super::{Avatar, World},
    implements::{Close, Dig, Drop, MeleeAttack, Open, Read, Skip, Walk, Wear, WieldFromGround},
    Action, ActionImpl, ActionPossibility,
};

#[enum_dispatch(ActionImpl)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    Skip,
    Walk,
    WieldFromGround,
    Drop,
    Dig,
    Read,
    Open,
    Close,
    Wear,
    MeleeAttack,
}
