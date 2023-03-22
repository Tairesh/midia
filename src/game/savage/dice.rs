use std::ops::{Add, AddAssign, Sub, SubAssign};

use enum_iterator::{next, previous, Sequence};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::traits::Name;

// TODO: it's getting a bit messy here, maybe it's time to split this file

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct RollResult {
    pub natural: u8,
    pub total: i8,
}

impl RollResult {
    pub fn new(natural: u8, total: i8) -> Self {
        RollResult { natural, total }
    }

    pub fn successes(self) -> u8 {
        self.total.max(0) as u8 / 4
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct DiceWithModifier(Dice, i8);

impl DiceWithModifier {
    pub fn new(dice: Dice, modifier: i8) -> Self {
        DiceWithModifier(dice, modifier)
    }

    pub fn no_modifier(dice: Dice) -> Self {
        DiceWithModifier(dice, 0)
    }

    pub fn roll(self) -> RollResult {
        let natural = self.0.roll();
        RollResult::new(natural, natural as i8 + self.1)
    }

    pub fn roll_explosive(self) -> RollResult {
        let natural = self.0.roll_explosive();
        RollResult::new(natural, natural as i8 + self.1)
    }

    pub fn value(self) -> u8 {
        (self.0.value() as i8 + self.1).max(0) as u8
    }

    pub fn with_modifier(self, modifier: i8) -> Self {
        DiceWithModifier(self.0, self.1 + modifier)
    }

    pub fn dice(self) -> Dice {
        self.0
    }

    pub fn modifier(self) -> i8 {
        self.1
    }
}

impl From<DiceWithModifier> for String {
    fn from(dice: DiceWithModifier) -> Self {
        let dice_name = dice.0.name().to_string();
        match dice.1 {
            0 => dice_name,
            _ => dice_name + if dice.1 > 0 { "+" } else { "" } + format!("{}", dice.1).as_str(),
        }
    }
}

impl From<Dice> for DiceWithModifier {
    fn from(dice: Dice) -> Self {
        Self::no_modifier(dice)
    }
}

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

    pub fn steps_above_attr(self, attr: Dice) -> i8 {
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
    use super::{Dice, SkillLevel};

    #[test]
    fn test_steps_above() {
        assert_eq!(SkillLevel::None.steps_above(SkillLevel::None), 0);
        assert_eq!(SkillLevel::D4.steps_above(SkillLevel::None), 1);
        assert_eq!(SkillLevel::D6.steps_above(SkillLevel::None), 2);
        assert_eq!(SkillLevel::D8.steps_above(SkillLevel::None), 3);
        assert_eq!(SkillLevel::D10.steps_above(SkillLevel::None), 4);
        assert_eq!(SkillLevel::D12.steps_above(SkillLevel::None), 5);
        assert_eq!(SkillLevel::None.steps_above(SkillLevel::D4), -1);
        assert_eq!(SkillLevel::D4.steps_above(SkillLevel::D4), 0);
        assert_eq!(SkillLevel::D6.steps_above(SkillLevel::D4), 1);
        assert_eq!(SkillLevel::D8.steps_above(SkillLevel::D4), 2);
        assert_eq!(SkillLevel::D10.steps_above(SkillLevel::D4), 3);
        assert_eq!(SkillLevel::D12.steps_above(SkillLevel::D4), 4);
        assert_eq!(SkillLevel::None.steps_above(SkillLevel::D6), -2);
        assert_eq!(SkillLevel::D4.steps_above(SkillLevel::D6), -1);
        assert_eq!(SkillLevel::D6.steps_above(SkillLevel::D6), 0);
        assert_eq!(SkillLevel::D8.steps_above(SkillLevel::D6), 1);
        assert_eq!(SkillLevel::D10.steps_above(SkillLevel::D6), 2);
        assert_eq!(SkillLevel::D12.steps_above(SkillLevel::D6), 3);
        assert_eq!(SkillLevel::None.steps_above(SkillLevel::D8), -3);
        assert_eq!(SkillLevel::D4.steps_above(SkillLevel::D8), -2);
        assert_eq!(SkillLevel::D6.steps_above(SkillLevel::D8), -1);
        assert_eq!(SkillLevel::D8.steps_above(SkillLevel::D8), 0);
        assert_eq!(SkillLevel::D10.steps_above(SkillLevel::D8), 1);
        assert_eq!(SkillLevel::D12.steps_above(SkillLevel::D8), 2);
        assert_eq!(SkillLevel::None.steps_above(SkillLevel::D10), -4);
        assert_eq!(SkillLevel::D4.steps_above(SkillLevel::D10), -3);
        assert_eq!(SkillLevel::D6.steps_above(SkillLevel::D10), -2);
        assert_eq!(SkillLevel::D8.steps_above(SkillLevel::D10), -1);
        assert_eq!(SkillLevel::D10.steps_above(SkillLevel::D10), 0);
        assert_eq!(SkillLevel::D12.steps_above(SkillLevel::D10), 1);
        assert_eq!(SkillLevel::None.steps_above(SkillLevel::D12), -5);
        assert_eq!(SkillLevel::D4.steps_above(SkillLevel::D12), -4);
        assert_eq!(SkillLevel::D6.steps_above(SkillLevel::D12), -3);
        assert_eq!(SkillLevel::D8.steps_above(SkillLevel::D12), -2);
        assert_eq!(SkillLevel::D10.steps_above(SkillLevel::D12), -1);
        assert_eq!(SkillLevel::D12.steps_above(SkillLevel::D12), 0);
    }

    #[test]
    fn test_steps_above_attr() {
        assert_eq!(SkillLevel::None.steps_above_attr(Dice::D4), -1);
        assert_eq!(SkillLevel::D4.steps_above_attr(Dice::D4), 0);
        assert_eq!(SkillLevel::D6.steps_above_attr(Dice::D4), 1);
        assert_eq!(SkillLevel::D8.steps_above_attr(Dice::D4), 2);
        assert_eq!(SkillLevel::D10.steps_above_attr(Dice::D4), 3);
        assert_eq!(SkillLevel::D12.steps_above_attr(Dice::D4), 4);
        assert_eq!(SkillLevel::None.steps_above_attr(Dice::D6), -2);
        assert_eq!(SkillLevel::D4.steps_above_attr(Dice::D6), -1);
        assert_eq!(SkillLevel::D6.steps_above_attr(Dice::D6), 0);
        assert_eq!(SkillLevel::D8.steps_above_attr(Dice::D6), 1);
        assert_eq!(SkillLevel::D10.steps_above_attr(Dice::D6), 2);
        assert_eq!(SkillLevel::D12.steps_above_attr(Dice::D6), 3);
        assert_eq!(SkillLevel::None.steps_above_attr(Dice::D8), -3);
        assert_eq!(SkillLevel::D4.steps_above_attr(Dice::D8), -2);
        assert_eq!(SkillLevel::D6.steps_above_attr(Dice::D8), -1);
        assert_eq!(SkillLevel::D8.steps_above_attr(Dice::D8), 0);
        assert_eq!(SkillLevel::D10.steps_above_attr(Dice::D8), 1);
        assert_eq!(SkillLevel::D12.steps_above_attr(Dice::D8), 2);
        assert_eq!(SkillLevel::None.steps_above_attr(Dice::D10), -4);
        assert_eq!(SkillLevel::D4.steps_above_attr(Dice::D10), -3);
        assert_eq!(SkillLevel::D6.steps_above_attr(Dice::D10), -2);
        assert_eq!(SkillLevel::D8.steps_above_attr(Dice::D10), -1);
        assert_eq!(SkillLevel::D10.steps_above_attr(Dice::D10), 0);
        assert_eq!(SkillLevel::D12.steps_above_attr(Dice::D10), 1);
        assert_eq!(SkillLevel::None.steps_above_attr(Dice::D12), -5);
        assert_eq!(SkillLevel::D4.steps_above_attr(Dice::D12), -4);
        assert_eq!(SkillLevel::D6.steps_above_attr(Dice::D12), -3);
        assert_eq!(SkillLevel::D8.steps_above_attr(Dice::D12), -2);
        assert_eq!(SkillLevel::D10.steps_above_attr(Dice::D12), -1);
        assert_eq!(SkillLevel::D12.steps_above_attr(Dice::D12), 0);
    }
}
