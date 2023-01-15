use crate::game::races::Race;

use super::{Attribute, Attributes, DiceWithModifier, Skill, Skills, Wound};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CharSheet {
    pub race: Race,
    pub age: u8,
    pub attributes: Attributes,
    pub skills: Skills,
    pub wounds: Vec<Wound>,
}

impl CharSheet {
    pub fn new(race: Race, age: u8, attributes: Attributes, skills: Skills) -> Self {
        Self {
            race,
            age,
            attributes,
            skills,
            wounds: vec![],
        }
    }

    pub fn default(race: Race, age: u8) -> Self {
        Self::new(race, age, Attributes::default(), Skills::default(race))
    }

    pub fn random(race: Race, age: u8) -> Self {
        let attributes = Attributes::random();
        let skills = Skills::random(&attributes, race);
        Self::new(race, age, attributes, skills)
    }

    pub fn calc_skill_points(&self) -> i8 {
        self.skills.calc_skill_points(&self.attributes, self.race)
    }

    pub fn parry(&self) -> u8 {
        2 + self.get_skill_with_modifiers(Skill::Fighting).value() / 2
    }

    pub fn toughness(&self) -> u8 {
        (2 + self.get_attribute_with_modifiers(Attribute::Vigor).value() as i8 / 2
            + self.race.toughness_bonus())
        .max(0) as u8
    }

    pub fn walk_koeff(&self) -> f32 {
        // TODO: write tests for moving for different races and ages
        let k_age = match self.age {
            0 => 100.0,
            1..=3 => 10.0,
            4..=10 => 3.0,
            11.. => 1.0,
        };
        let mut k_wounds = 1.0;
        for &wound in &self.wounds {
            if wound == Wound::LeftLeg || wound == Wound::RightLeg {
                k_wounds *= 0.75;
            }
        }

        k_age * k_wounds * self.race.walk_koeff()
    }

    pub fn get_attribute_with_modifiers(&self, attribute: Attribute) -> DiceWithModifier {
        let mut value = self.attributes.get_attribute(attribute);
        match attribute {
            Attribute::Vigor => {
                if self.wounds.contains(&Wound::BatteredGuts) {
                    value -= 1;
                }
            }
            Attribute::Agility => {
                if self.wounds.contains(&Wound::BrokenGuts) {
                    value -= 1;
                }
            }
            Attribute::Strength => {
                if self.wounds.contains(&Wound::BustedGuts) {
                    value -= 1;
                }
            }
            Attribute::Smarts => {
                if self.wounds.contains(&Wound::BrainDamage) {
                    value -= 1;
                }
            }
            Attribute::Spirit => {}
        }
        value.into()
    }

    pub fn get_skill_with_modifiers(&self, skill: Skill) -> DiceWithModifier {
        self.skills.get_skill(skill).into()
    }
}
