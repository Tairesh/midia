use serde::{Deserialize, Serialize};

use crate::game::traits::Name;

use super::{Dice, RollResult};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct DiceWithModifier(pub Dice, pub i8);

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
