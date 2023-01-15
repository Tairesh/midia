use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::colors::Colors;

use super::Dice;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attributes {
    /// Physical precision and speed
    pub agility: Dice,
    /// Mental power
    pub smarts: Dice,
    /// Willpower
    pub spirit: Dice,
    /// Physical power
    pub strength: Dice,
    /// Physical health
    pub vigor: Dice,
}

impl Attributes {
    pub fn random() -> Self {
        let mut attributes = Self::default();
        let mut points = 5;
        while points > 0 {
            let random_attr = rand::thread_rng().gen::<Attribute>();
            attributes.set_attribute(random_attr, attributes.get_attribute(random_attr) + 1);
            points -= 1;
        }

        attributes
    }

    pub fn get_attribute(&self, attribute: Attribute) -> Dice {
        match attribute {
            Attribute::Agility => self.agility,
            Attribute::Smarts => self.smarts,
            Attribute::Spirit => self.spirit,
            Attribute::Strength => self.strength,
            Attribute::Vigor => self.vigor,
        }
    }

    pub fn set_attribute(&mut self, attribute: Attribute, value: Dice) {
        match attribute {
            Attribute::Agility => self.agility = value,
            Attribute::Smarts => self.smarts = value,
            Attribute::Spirit => self.spirit = value,
            Attribute::Strength => self.strength = value,
            Attribute::Vigor => self.vigor = value,
        }
    }

    pub fn get_attributes(&self) -> Vec<(Attribute, Dice)> {
        Attribute::iterator()
            .map(move |attr| (attr, self.get_attribute(attr)))
            .collect()
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            agility: Dice::D4,
            smarts: Dice::D4,
            spirit: Dice::D4,
            strength: Dice::D4,
            vigor: Dice::D4,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Attribute {
    Agility,
    Smarts,
    Spirit,
    Strength,
    Vigor,
}

impl Attribute {
    const AGILITY_COLOR: Color = Colors::LIME_GREEN;
    const SMARTS_COLOR: Color = Colors::LIGHT_SKY_BLUE;
    const SPIRIT_COLOR: Color = Colors::LIGHT_GOLDEN_ROD_YELLOW;
    const STRENGTH_COLOR: Color = Colors::ORANGE_RED;
    const VIGOR_COLOR: Color = Colors::VIOLET;

    pub fn color(self) -> Color {
        match self {
            Attribute::Agility => Self::AGILITY_COLOR,
            Attribute::Smarts => Self::SMARTS_COLOR,
            Attribute::Spirit => Self::SPIRIT_COLOR,
            Attribute::Strength => Self::STRENGTH_COLOR,
            Attribute::Vigor => Self::VIGOR_COLOR,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Attribute::Agility => "Agility",
            Attribute::Smarts => "Smarts",
            Attribute::Spirit => "Spirit",
            Attribute::Strength => "Strength",
            Attribute::Vigor => "Vigor",
        }
    }

    pub fn iterator() -> impl Iterator<Item = Attribute> {
        vec![
            Attribute::Agility,
            Attribute::Smarts,
            Attribute::Spirit,
            Attribute::Strength,
            Attribute::Vigor,
        ]
        .into_iter()
    }
}

impl Distribution<Attribute> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Attribute {
        unsafe { std::mem::transmute(rng.gen::<u8>() % Attribute::iterator().count() as u8) }
    }
}
