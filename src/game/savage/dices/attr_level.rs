use std::ops::{Add, AddAssign, Sub, SubAssign};

use enum_iterator::{next, previous, Sequence};
use serde::{Deserialize, Serialize};
use tetra::math::num_traits::Signed;

use crate::game::traits::Name;
use crate::game::SkillLevel;

use super::{Dice, DiceWithModifier, RollResult};

#[derive(
    Serialize, Deserialize, Sequence, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default,
)]
pub enum AttrLevel {
    #[default]
    D4,
    D6,
    D8,
    D10,
    D12,
    #[serde(rename = "d12+1")]
    D12Plus1,
    #[serde(rename = "d12+2")]
    D12Plus2,
}

impl From<AttrLevel> for Dice {
    fn from(value: AttrLevel) -> Self {
        DiceWithModifier::from(value).0
    }
}

impl From<AttrLevel> for DiceWithModifier {
    fn from(value: AttrLevel) -> Self {
        match value {
            AttrLevel::D4 => Dice::D4.into(),
            AttrLevel::D6 => Dice::D6.into(),
            AttrLevel::D8 => Dice::D8.into(),
            AttrLevel::D10 => Dice::D10.into(),
            AttrLevel::D12 => Dice::D12.into(),
            AttrLevel::D12Plus1 => DiceWithModifier::new(Dice::D12, 1),
            AttrLevel::D12Plus2 => DiceWithModifier::new(Dice::D12, 2),
        }
    }
}

impl From<AttrLevel> for &str {
    fn from(value: AttrLevel) -> Self {
        match value {
            AttrLevel::D4 => "d4",
            AttrLevel::D6 => "d6",
            AttrLevel::D8 => "d8",
            AttrLevel::D10 => "d10",
            AttrLevel::D12 => "d12",
            AttrLevel::D12Plus1 => "d12+1",
            AttrLevel::D12Plus2 => "d12+2",
        }
    }
}

impl Name for AttrLevel {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

impl AttrLevel {
    pub fn value(self) -> u8 {
        match self {
            AttrLevel::D4 => 4,
            AttrLevel::D6 => 6,
            AttrLevel::D8 => 8,
            AttrLevel::D10 => 10,
            AttrLevel::D12 => 12,
            AttrLevel::D12Plus1 => 13,
            AttrLevel::D12Plus2 => 14,
        }
    }

    pub fn roll(self) -> u8 {
        Dice::from(self).roll()
    }

    pub fn roll_explosive(self) -> u8 {
        Dice::from(self).roll_explosive()
    }

    pub fn next(self) -> Option<Self> {
        next(&self)
    }

    pub fn prev(self) -> Option<Self> {
        previous(&self)
    }
}

impl Add<i8> for AttrLevel {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        match rhs {
            0 => self,
            1 => self.next().unwrap_or(self),
            -1 => self.prev().unwrap_or(self),
            _ => self + rhs.signum(),
        }
    }
}

impl AddAssign<i8> for AttrLevel {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<i8> for AttrLevel {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign<i8> for AttrLevel {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl From<Dice> for AttrLevel {
    fn from(value: Dice) -> Self {
        match value {
            Dice::D4 => Self::D4,
            Dice::D6 => Self::D6,
            Dice::D8 => Self::D8,
            Dice::D10 => Self::D10,
            Dice::D12 => Self::D12,
        }
    }
}

impl From<AttrLevel> for SkillLevel {
    fn from(value: AttrLevel) -> Self {
        match value {
            AttrLevel::D4 => SkillLevel::D4,
            AttrLevel::D6 => SkillLevel::D6,
            AttrLevel::D8 => SkillLevel::D8,
            AttrLevel::D10 => SkillLevel::D10,
            _ => SkillLevel::D12,
        }
    }
}

impl From<SkillLevel> for AttrLevel {
    fn from(value: SkillLevel) -> Self {
        match value {
            SkillLevel::None | SkillLevel::D4 => AttrLevel::D4,
            SkillLevel::D6 => AttrLevel::D6,
            SkillLevel::D8 => AttrLevel::D8,
            SkillLevel::D10 => AttrLevel::D10,
            SkillLevel::D12 => AttrLevel::D12,
        }
    }
}
