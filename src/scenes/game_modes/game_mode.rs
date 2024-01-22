use enum_dispatch::enum_dispatch;

use super::implements::{
    Closing, Digging, Dropping, Examining, ForceAttack, Observing, Opening, Reading, Shooting,
    Throwing, Walking, WieldingFromGround,
};

#[enum_dispatch(GameModeImpl)]
pub enum GameMode {
    Walking,
    Examining,
    WieldingFromGround,
    Dropping,
    Digging,
    Observing,
    Reading,
    Opening,
    Closing,
    ForceAttack,
    Throwing,
    Shooting,
}
