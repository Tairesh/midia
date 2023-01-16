use enum_dispatch::enum_dispatch;

use super::implements::{
    Closing, Digging, Dropping, Examining, ForceAttack, Observing, Opening, Reading, Walking,
    WieldingFromGround,
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
}
