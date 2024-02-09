use std::ops::{Add, AddAssign, Sub, SubAssign};

use enum_iterator::{next, previous, Sequence};
use serde::{Deserialize, Serialize};
use tetra::math::num_traits::Signed;

use crate::game::traits::Name;

use super::{AttrLevel, Dice, DiceWithModifier, RollResult};

#[derive(
    Serialize, Deserialize, Sequence, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default,
)]
pub enum SkillLevel {
    #[default]
    #[serde(rename = "d4-2")]
    None,
    D4,
    D6,
    D8,
    D10,
    D12,
}

impl From<SkillLevel> for DiceWithModifier {
    fn from(skill_level: SkillLevel) -> Self {
        match skill_level {
            SkillLevel::None => DiceWithModifier::new(Dice::D4, -2),
            SkillLevel::D4 => Dice::D4.into(),
            SkillLevel::D6 => Dice::D6.into(),
            SkillLevel::D8 => Dice::D8.into(),
            SkillLevel::D10 => Dice::D10.into(),
            SkillLevel::D12 => Dice::D12.into(),
        }
    }
}

impl From<SkillLevel> for Dice {
    fn from(skill_level: SkillLevel) -> Self {
        DiceWithModifier::from(skill_level).0
    }
}

impl From<SkillLevel> for &str {
    fn from(skill_level: SkillLevel) -> Self {
        match skill_level {
            SkillLevel::None => "d4-2",
            SkillLevel::D4 => "d4",
            SkillLevel::D6 => "d6",
            SkillLevel::D8 => "d8",
            SkillLevel::D10 => "d10",
            SkillLevel::D12 => "d12",
        }
    }
}

impl Name for SkillLevel {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

impl SkillLevel {
    pub fn value(self) -> u8 {
        match self {
            SkillLevel::None => 2,
            SkillLevel::D4 => 4,
            SkillLevel::D6 => 6,
            SkillLevel::D8 => 8,
            SkillLevel::D10 => 10,
            SkillLevel::D12 => 12,
        }
    }

    pub fn roll(self) -> RollResult {
        DiceWithModifier::from(self).roll()
    }

    pub fn roll_explosive(self) -> RollResult {
        DiceWithModifier::from(self).roll_explosive()
    }

    pub fn next(self) -> Option<Self> {
        next(&self)
    }

    pub fn prev(self) -> Option<Self> {
        previous(&self)
    }

    pub fn steps_above(self, other: Self) -> i8 {
        self as i8 - other as i8
    }

    pub fn steps_above_attr(self, attr: AttrLevel) -> i8 {
        self as i8 - (attr as i8 + 1)
    }
}

impl Add<i8> for SkillLevel {
    type Output = SkillLevel;

    fn add(self, rhs: i8) -> Self::Output {
        match rhs {
            0 => self,
            1 => self.next().unwrap_or(self),
            -1 => self.prev().unwrap_or(self),
            _ => self + rhs.signum(),
        }
    }
}

impl AddAssign<i8> for SkillLevel {
    fn add_assign(&mut self, rhs: i8) {
        *self = *self + rhs;
    }
}

impl Sub<i8> for SkillLevel {
    type Output = SkillLevel;

    fn sub(self, rhs: i8) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign<i8> for SkillLevel {
    fn sub_assign(&mut self, rhs: i8) {
        *self = *self - rhs;
    }
}

impl From<Dice> for SkillLevel {
    fn from(value: Dice) -> Self {
        match value {
            Dice::D4 => SkillLevel::D4,
            Dice::D6 => SkillLevel::D6,
            Dice::D8 => SkillLevel::D8,
            Dice::D10 => SkillLevel::D10,
            Dice::D12 => SkillLevel::D12,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(SkillLevel::None, SkillLevel::None, 0)]
    #[test_case(SkillLevel::D4, SkillLevel::None, 1)]
    #[test_case(SkillLevel::None, SkillLevel::D4, - 1)]
    #[test_case(SkillLevel::D4, SkillLevel::D4, 0)]
    #[test_case(SkillLevel::D6, SkillLevel::D4, 1)]
    #[test_case(SkillLevel::D4, SkillLevel::D6, - 1)]
    #[test_case(SkillLevel::D6, SkillLevel::D6, 0)]
    #[test_case(SkillLevel::D8, SkillLevel::D6, 1)]
    #[test_case(SkillLevel::D6, SkillLevel::D8, - 1)]
    #[test_case(SkillLevel::D8, SkillLevel::D8, 0)]
    #[test_case(SkillLevel::D10, SkillLevel::D8, 1)]
    #[test_case(SkillLevel::D8, SkillLevel::D10, - 1)]
    #[test_case(SkillLevel::D10, SkillLevel::D10, 0)]
    #[test_case(SkillLevel::D12, SkillLevel::D10, 1)]
    #[test_case(SkillLevel::D10, SkillLevel::D12, - 1)]
    #[test_case(SkillLevel::D12, SkillLevel::D12, 0)]
    fn test_steps_above(skill: SkillLevel, other: SkillLevel, expect: i8) {
        assert_eq!(skill.steps_above(other), expect);
    }

    #[test_case(SkillLevel::None, AttrLevel::D4, - 1)]
    #[test_case(SkillLevel::D4, AttrLevel::D4, 0)]
    #[test_case(SkillLevel::D6, AttrLevel::D4, 1)]
    #[test_case(SkillLevel::D4, AttrLevel::D6, - 1)]
    #[test_case(SkillLevel::D6, AttrLevel::D6, 0)]
    #[test_case(SkillLevel::D8, AttrLevel::D6, 1)]
    #[test_case(SkillLevel::D6, AttrLevel::D8, - 1)]
    #[test_case(SkillLevel::D8, AttrLevel::D8, 0)]
    #[test_case(SkillLevel::D10, AttrLevel::D8, 1)]
    #[test_case(SkillLevel::D8, AttrLevel::D10, - 1)]
    #[test_case(SkillLevel::D10, AttrLevel::D10, 0)]
    #[test_case(SkillLevel::D12, AttrLevel::D10, 1)]
    #[test_case(SkillLevel::D10, AttrLevel::D12, - 1)]
    #[test_case(SkillLevel::D12, AttrLevel::D12, 0)]
    fn test_steps_above_attr(skill: SkillLevel, attr: AttrLevel, expect: i8) {
        assert_eq!(skill.steps_above_attr(attr), expect);
    }
}
