use enum_dispatch::enum_dispatch;

use super::implements::{
    Closing, Digging, Dropping, Examining, MeleeAttack, Observing, Opening, PickingUp, PikeAttack,
    Reading, Shooting, Throwing, Walking,
};

#[enum_dispatch(GameModeImpl)]
pub enum GameMode {
    Walking,
    Examining,
    PickingUp,
    Dropping,
    Digging,
    Observing,
    Reading,
    Opening,
    Closing,
    MeleeAttack,
    PikeAttack,
    Throwing,
    Shooting,
}
