use crate::game::races::Race;
use crate::game::{Dice, SkillLevel};

use super::{Attributes, Skills};

fn calc_parry(skills: &Skills) -> u8 {
    2 + skills.fighting.value() / 2
}

fn calc_toughness(race: Race, attributes: &Attributes) -> u8 {
    (2 + attributes.vigor.value() as i8 / 2 + race.toughness_bonus()).max(0) as u8
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CharSheet {
    pub attributes: Attributes,
    pub skills: Skills,
    pub parry: u8,
    pub toughness: u8,
}

impl CharSheet {
    pub fn default(race: Race) -> Self {
        let attributes = Attributes::default();
        let skills = Skills::default(race);
        Self {
            parry: calc_parry(&skills),
            toughness: calc_toughness(race, &attributes),
            attributes,
            skills,
        }
    }

    pub fn random(race: Race) -> Self {
        let attributes = Attributes::random();
        // TODO: randomize skills
        let skills = Skills::default(race);
        Self {
            parry: calc_parry(&skills),
            toughness: calc_toughness(race, &attributes),
            attributes,
            skills,
        }
    }

    pub fn calc_skill_points(&self, race: Race) -> u8 {
        let mut skill_points = 15;
        for (attr, skill, value) in self.skills.get_skills_by_attributes() {
            let mut attr_value = self.attributes.get_attribute(attr);
            let mut base_value = SkillLevel::D4_2;
            if let Some(&free_skill_level) = race.free_skills().get(&skill) {
                if value == free_skill_level {
                    continue;
                }
                attr_value = Dice::max(free_skill_level.into(), attr_value);
                base_value = free_skill_level;
            }
            skill_points -=
                (value as i8 - base_value as i8) + value.steps_above_attr(attr_value).max(0);
        }

        skill_points.max(0) as u8
    }

    pub fn recalculate(&mut self, race: Race) {
        self.parry = calc_parry(&self.skills);
        self.toughness = calc_toughness(race, &self.attributes);
    }
}
