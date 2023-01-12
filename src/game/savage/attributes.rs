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
            match rand::thread_rng().gen::<u8>() % 5 {
                0 => {
                    attributes.agility += 1;
                }
                1 => {
                    attributes.smarts += 1;
                }
                2 => {
                    attributes.spirit += 1;
                }
                3 => {
                    attributes.strength += 1;
                }
                4 => {
                    attributes.vigor += 1;
                }
                _ => unreachable!(),
            }
            points -= 1;
        }

        attributes
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
