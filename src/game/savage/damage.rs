use serde::{Deserialize, Serialize};

use crate::game::CharSheet;

use super::{Attribute, Dice};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DamageDice {
    #[serde(rename = "D4/2")]
    D4Half,
    D4,
    #[serde(rename = "D6/2")]
    D6Half,
    D6,
    #[serde(rename = "D8/2")]
    D8Half,
    D8,
    #[serde(rename = "D10/2")]
    D10Half,
    D10,
    #[serde(rename = "D12/2")]
    D12Half,
    D12,
}

impl DamageDice {
    pub fn roll(self) -> u8 {
        match self {
            DamageDice::D4Half => Dice::D4.roll() / 2,
            DamageDice::D4 => Dice::D4.roll(),
            DamageDice::D6Half => Dice::D6.roll() / 2,
            DamageDice::D6 => Dice::D6.roll(),
            DamageDice::D8Half => Dice::D8.roll() / 2,
            DamageDice::D8 => Dice::D8.roll(),
            DamageDice::D10Half => Dice::D10.roll() / 2,
            DamageDice::D10 => Dice::D10.roll(),
            DamageDice::D12Half => Dice::D12.roll() / 2,
            DamageDice::D12 => Dice::D12.roll(),
        }
    }

    pub fn roll_explosive(self) -> u8 {
        match self {
            DamageDice::D4Half => Dice::D4.roll_explosive() / 2,
            DamageDice::D4 => Dice::D4.roll_explosive(),
            DamageDice::D6Half => Dice::D6.roll_explosive() / 2,
            DamageDice::D6 => Dice::D6.roll_explosive(),
            DamageDice::D8Half => Dice::D8.roll_explosive() / 2,
            DamageDice::D8 => Dice::D8.roll_explosive(),
            DamageDice::D10Half => Dice::D10.roll_explosive() / 2,
            DamageDice::D10 => Dice::D10.roll_explosive(),
            DamageDice::D12Half => Dice::D12.roll_explosive() / 2,
            DamageDice::D12 => Dice::D12.roll_explosive(),
        }
    }
}

impl From<Dice> for DamageDice {
    fn from(dice: Dice) -> Self {
        match dice {
            Dice::D4 => DamageDice::D4,
            Dice::D6 => DamageDice::D6,
            Dice::D8 => DamageDice::D8,
            Dice::D10 => DamageDice::D10,
            Dice::D12 => DamageDice::D12,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub dices: Vec<DamageDice>,
    #[serde(default)]
    pub attribute: Option<Attribute>,
    #[serde(default)]
    pub modifier: i8,
}

impl Damage {
    pub fn roll(
        &self,
        char_sheet: &CharSheet,
        critical: bool,
        explosive: bool,
        minimum_strength: Option<Dice>,
    ) -> u8 {
        let mut not_enough_strength = false;
        let strength = char_sheet
            .get_attribute_with_modifiers(Attribute::Strength)
            .dice();
        if let Some(minimum_strength) = minimum_strength {
            if strength < minimum_strength {
                not_enough_strength = true;
            }
        }

        let mut result = self.modifier;
        if not_enough_strength && result > 0 {
            result = 0;
        }
        for mut dice in self.dices.iter().copied() {
            if not_enough_strength {
                let strength = strength.into();
                if strength < dice {
                    dice = strength;
                }
            }
            result += if explosive {
                dice.roll_explosive()
            } else {
                dice.roll()
            } as i8;
        }
        if let Some(attribute) = self.attribute {
            let attribute_dice = char_sheet.get_attribute_with_modifiers(attribute);
            result += if explosive {
                attribute_dice.roll_explosive()
            } else {
                attribute_dice.roll()
            }
            .total;
        }
        if critical {
            result += if explosive {
                Dice::D6.roll_explosive()
            } else {
                Dice::D6.roll()
            } as i8;
        }
        result.max(0) as u8
    }
}
