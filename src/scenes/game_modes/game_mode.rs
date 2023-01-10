use enum_dispatch::enum_dispatch;

use super::implements::{Digging, Dropping, Examining, Observing, Reading, Walking, Wielding};

#[enum_dispatch(GameModeImpl)]
pub enum GameMode {
    Walking,
    Examining,
    Wielding,
    Dropping,
    Digging,
    Observing,
    Reading,
}
