use serde::{Deserialize, Serialize};

use crate::game::CharSheet;

use super::{Attribute, Dice};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub dices: Vec<Dice>,
    #[serde(default)]
    pub attribute: Option<Attribute>,
    #[serde(default)]
    pub modifier: i8,
}

impl Damage {
    pub fn roll(&self, char_sheet: &CharSheet) -> u8 {
        let mut result = self.modifier;
        for dice in &self.dices {
            result += dice.roll_wild() as i8;
        }
        if let Some(attribute) = self.attribute {
            result += char_sheet
                .get_attribute_with_modifiers(attribute)
                .roll_wild() as i8;
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
