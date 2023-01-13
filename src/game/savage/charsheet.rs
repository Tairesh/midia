use crate::game::races::Race;
use crate::game::{Dice, SkillLevel};

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
                (value as u8 - base_value as u8) + value.steps_above_attr(attr_value).max(0) as u8;
        }

        skill_points
    }
}
