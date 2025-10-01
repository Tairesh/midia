#![allow(dead_code)]
use super::Dice;

pub struct DiceStack(Vec<Dice>);

impl DiceStack {
    pub fn new(dices: Vec<Dice>) -> Self {
        Self(dices)
    }

    pub fn from_dice(dice: Dice) -> Self {
        Self(vec![dice])
    }

    pub fn push(&mut self, dice: Dice) {
        self.0.push(dice);
    }

    pub fn pop(&mut self) -> Option<Dice> {
        self.0.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn new_2d6() -> Self {
        Self::new(vec![Dice::D6, Dice::D6])
    }

    pub fn roll_all(&self) -> Vec<u8> {
        self.0.iter().map(|d| d.roll()).collect()
    }

    pub fn roll_all_explosive(&self) -> Vec<u8> {
        self.0.iter().map(|d| d.roll_explosive()).collect()
    }

    pub fn roll_total(&self) -> u8 {
        self.0.iter().map(|d| d.roll()).sum()
    }

    pub fn roll_total_explosive(&self) -> u8 {
        self.0.iter().map(|d| d.roll_explosive()).sum()
    }

    pub fn damage(&self) -> u8 {
        self.roll_total_explosive()
    }
}

impl From<Dice> for DiceStack {
    fn from(dice: Dice) -> Self {
        Self::from_dice(dice)
    }
}

impl From<Vec<Dice>> for DiceStack {
    fn from(dices: Vec<Dice>) -> Self {
        Self::new(dices)
    }
}

impl From<(Dice, Dice)> for DiceStack {
    fn from(dices: (Dice, Dice)) -> Self {
        Self::new(vec![dices.0, dices.1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_stack() {
        let mut stack = DiceStack::new_2d6();

        assert_eq!(stack.len(), 2);
        assert!(!stack.is_empty());

        let roll = stack.roll_all();
        assert_eq!(roll.len(), 2);
        for r in roll {
            assert!(r >= 1 && r <= 6);
        }

        let total = stack.roll_total();
        assert!(total >= 2 && total <= 12);

        stack.push(Dice::D4);
        assert_eq!(stack.len(), 3);
        let total = stack.roll_total();
        assert!(total >= 3 && total <= 16);

        let pop = stack.pop();
        assert_eq!(pop, Some(Dice::D4));
        assert_eq!(stack.len(), 2);
    }
}
