use super::Dice;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            agility: Dice::D6,
            smarts: Dice::D6,
            spirit: Dice::D6,
            strength: Dice::D6,
            vigor: Dice::D6,
        }
    }
}
