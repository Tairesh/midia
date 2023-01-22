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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub dices: Vec<DamageDice>,
    #[serde(default)]
    pub attribute: Option<Attribute>,
    #[serde(default)]
    pub modifier: i8,
}

impl Damage {
    pub fn roll(&self, char_sheet: &CharSheet, critical: bool, explosive: bool) -> u8 {
        let mut result = self.modifier;
        for dice in &self.dices {
            result += if explosive {
                dice.roll_explosive()
            } else {
                dice.roll()
            } as i8;
        }
        if let Some(attribute) = self.attribute {
            result += char_sheet
                .get_attribute_with_modifiers(attribute)
                .roll_explosive() as i8;
        }
        if critical {
            result += Dice::D6.roll_explosive() as i8;
        }
        result.max(0) as u8
    }
}

impl Default for Damage {
    fn default() -> Self {
        Self {
            dices: vec![],
            attribute: Some(Attribute::Strength),
            modifier: -2,
        }
    }
}
