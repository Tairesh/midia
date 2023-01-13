use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

use super::Dice;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Attribute {
    Agility,
    Smarts,
    Spirit,
    Strength,
    Vigor,
}

impl Distribution<Attribute> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Attribute {
        match rng.gen::<u8>() % 5 {
            0 => Attribute::Agility,
            1 => Attribute::Smarts,
            2 => Attribute::Spirit,
            3 => Attribute::Strength,
            4 => Attribute::Vigor,
            _ => unreachable!(),
        }
    }
}
