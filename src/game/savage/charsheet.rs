use super::{Attributes, Skills};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct CharSheet {
    pub attributes: Attributes,
    pub skills: Skills,
}

impl CharSheet {
    pub fn random() -> Self {
        Self {
            attributes: Attributes::random(),
            skills: Skills::default(),
        }
    }

    pub fn calc_skill_points(&self) -> u8 {
        let mut skill_points = 15;

        let agility = self.attributes.agility;
        skill_points -= self.skills.athletics as u8 + (self.skills.athletics - agility);
        skill_points -= self.skills.fighting as u8 + (self.skills.fighting - agility);
        skill_points -= self.skills.shooting as u8 + (self.skills.shooting - agility);
        skill_points -= self.skills.stealth as u8 + (self.skills.stealth - agility);
        skill_points -= self.skills.thievery as u8 + (self.skills.thievery - agility);
        skill_points -= self.skills.swimming as u8 + (self.skills.swimming - agility);

        let smarts = self.attributes.smarts;
        skill_points -= self.skills.gambling as u8 + (self.skills.gambling - smarts);
        skill_points -= self.skills.notice as u8 + (self.skills.notice - smarts);
        skill_points -= self.skills.survival as u8 + (self.skills.survival - smarts);
        skill_points -= self.skills.healing as u8 + (self.skills.healing - smarts);
        skill_points -= self.skills.repair as u8 + (self.skills.repair - smarts);
        skill_points -= self.skills.reading as u8 + (self.skills.reading - smarts);

        let spirit = self.attributes.spirit;
        skill_points -= self.skills.persuasion as u8 + (self.skills.persuasion - spirit);
        skill_points -= self.skills.intimidation as u8 + (self.skills.intimidation - spirit);

        let strength = self.attributes.strength;
        skill_points -= self.skills.climbing as u8 + (self.skills.climbing - strength);

        skill_points
    }
}
