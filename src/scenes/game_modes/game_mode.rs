use enum_dispatch::enum_dispatch;

use super::implements::{
    Closing, Digging, Dropping, Examining, Observing, Opening, Reading, Walking, Wielding,
};

#[enum_dispatch(GameModeImpl)]
pub enum GameMode {
    Walking,
    Examining,
    Wielding,
    Dropping,
    Digging,
    Observing,
    Reading,
    Opening,
    Closing,
}
