use crate::game::races::Race;

use super::{Attribute, Attributes, DiceWithModifier, Skills, Wound};

fn calc_parry(skills: &Skills) -> u8 {
    2 + skills.fighting.value() / 2
}

fn calc_toughness(race: Race, attributes: &Attributes) -> u8 {
    (2 + attributes.vigor.value() as i8 / 2 + race.toughness_bonus()).max(0) as u8
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CharSheet {
    pub race: Race,
    pub age: u8,
    pub attributes: Attributes,
    pub skills: Skills,
    pub wounds: Vec<Wound>,
    pub parry: u8,
    pub toughness: u8,
}

impl CharSheet {
    pub fn new(race: Race, age: u8, attributes: Attributes, skills: Skills) -> Self {
        let parry = calc_parry(&skills);
        let toughness = calc_toughness(race, &attributes);
        Self {
            race,
            age,
            attributes,
            skills,
            wounds: vec![],
            parry,
            toughness,
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

    pub fn recalculate(&mut self) {
        self.parry = calc_parry(&self.skills);
        self.toughness = calc_toughness(self.race, &self.attributes);
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

    #[allow(dead_code)]
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
}
