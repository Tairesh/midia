use std::ops::{Add, AddAssign, Sub, SubAssign};

use enum_iterator::{next, previous, Sequence};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tetra::math::num_traits::Signed;

use crate::game::traits::Name;

// TODO: it's getting a bit messy here, maybe it's time to split this file
// TODO: use world's rng instead of thread_rng

#[derive(Serialize, Deserialize, Sequence, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
}

impl From<Dice> for &str {
    fn from(dice: Dice) -> Self {
        match dice {
            Dice::D4 => "d4",
            Dice::D6 => "d6",
            Dice::D8 => "d8",
            Dice::D10 => "d10",
            Dice::D12 => "d12",
        }
    }
}

impl Name for Dice {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

impl Dice {
    pub fn value(self) -> u8 {
        match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
        }
    }

    pub fn roll(self) -> u8 {
        // TODO: Use world's rng instead of thread_rng
        rand::thread_rng().gen::<u8>() % self.value() + 1
    }

    /// Roll a dice that explodes on the maximum value.
    pub fn roll_explosive(self) -> u8 {
        let mut total = 0u8;
        let mut roll = self.roll();
        while roll == self.value() {
            // probably u8 here is not an intelligent choice but who cares
            total = total.saturating_add(roll);
            roll = self.roll();
        }
        total + roll
    }

    pub fn next(self) -> Option<Self> {
        next(&self)
    }

    pub fn prev(self) -> Option<Self> {
        previous(&self)
    }
}

impl Add<i8> for Dice {
    type Output = Dice;

    fn add(self, rhs: i8) -> Self::Output {
        match rhs {
            0 => self,
            1 => self.next().unwrap_or(self),
            -1 => self.prev().unwrap_or(self),
            _ => self + rhs.signum(),
        }
    }
}

impl AddAssign<i8> for Dice {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<i8> for Dice {
    type Output = Dice;

    fn sub(self, rhs: i8) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign<i8> for Dice {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}
