use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::game::races::Race;
use crate::game::savage::Attributes;
use crate::game::Dice;

use super::{Attribute, SkillLevel};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Skills {
    pub athletics: SkillLevel,
    pub fighting: SkillLevel,
    pub shooting: SkillLevel,
    pub stealth: SkillLevel,
    pub thievery: SkillLevel,
    pub swimming: SkillLevel,
    pub gambling: SkillLevel,
    pub notice: SkillLevel,
    pub survival: SkillLevel,
    pub healing: SkillLevel,
    pub repair: SkillLevel,
    pub reading: SkillLevel,
    pub persuasion: SkillLevel,
    pub intimidation: SkillLevel,
    pub climbing: SkillLevel,
}

impl Skills {
    pub fn default(race: Race) -> Self {
        Self {
            athletics: SkillLevel::None,
            fighting: SkillLevel::None,
            shooting: SkillLevel::None,
            stealth: SkillLevel::None,
            thievery: SkillLevel::None,
            swimming: if race == Race::Totik {
                SkillLevel::D6
            } else {
                SkillLevel::None
            },
            gambling: SkillLevel::None,
            notice: SkillLevel::None,
            survival: SkillLevel::None,
            healing: SkillLevel::None,
            repair: SkillLevel::None,
            reading: SkillLevel::None,
            persuasion: SkillLevel::None,
            intimidation: SkillLevel::None,
            climbing: if race == Race::Gazan {
                SkillLevel::D6
            } else {
                SkillLevel::None
            },
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R, attributes: &Attributes, race: Race) -> Self {
        let mut skills = Self::default(race);

        let mut points = skills.calc_skill_points(attributes, race);
        while points > 0 {
            let random_skill = rng.gen::<Skill>();
            skills.set_skill(random_skill, skills.get_skill(random_skill) + 1);

            points = skills.calc_skill_points(attributes, race);
            if points < 0 {
                skills.set_skill(random_skill, skills.get_skill(random_skill) - 1);
                points = skills.calc_skill_points(attributes, race);
            }
        }

        skills
    }

    pub fn get_skill(&self, skill: Skill) -> SkillLevel {
        match skill {
            Skill::Athletics => self.athletics,
            Skill::Fighting => self.fighting,
            Skill::Shooting => self.shooting,
            Skill::Stealth => self.stealth,
            Skill::Thievery => self.thievery,
            Skill::Swimming => self.swimming,
            Skill::Gambling => self.gambling,
            Skill::Notice => self.notice,
            Skill::Survival => self.survival,
            Skill::Healing => self.healing,
            Skill::Repair => self.repair,
            Skill::Reading => self.reading,
            Skill::Persuasion => self.persuasion,
            Skill::Intimidation => self.intimidation,
            Skill::Climbing => self.climbing,
        }
    }

    pub fn set_skill(&mut self, skill: Skill, level: SkillLevel) {
        match skill {
            Skill::Athletics => self.athletics = level,
            Skill::Fighting => self.fighting = level,
            Skill::Shooting => self.shooting = level,
            Skill::Stealth => self.stealth = level,
            Skill::Thievery => self.thievery = level,
            Skill::Swimming => self.swimming = level,
            Skill::Gambling => self.gambling = level,
            Skill::Notice => self.notice = level,
            Skill::Survival => self.survival = level,
            Skill::Healing => self.healing = level,
            Skill::Repair => self.repair = level,
            Skill::Reading => self.reading = level,
            Skill::Persuasion => self.persuasion = level,
            Skill::Intimidation => self.intimidation = level,
            Skill::Climbing => self.climbing = level,
        }
    }

    pub fn get_skills_by_attributes(&self) -> Vec<(Attribute, Skill, SkillLevel)> {
        Skill::iterator()
            .map(|skill| (skill.attribute(), skill, self.get_skill(skill)))
            .collect()
    }

    pub fn calc_skill_points(&self, attributes: &Attributes, race: Race) -> i8 {
        let mut skill_points = 15;
        for (attr, skill, value) in self.get_skills_by_attributes() {
            let mut attr_value = attributes.get_attribute(attr);
            let mut base_value = SkillLevel::None;
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

        skill_points
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Skill {
    Athletics,
    Fighting,
    Shooting,
    Stealth,
    Thievery,
    Swimming,
    Gambling,
    Notice,
    Survival,
    Healing,
    Repair,
    Reading,
    Persuasion,
    Intimidation,
    Climbing,
}

impl Skill {
    pub fn name(self) -> &'static str {
        match self {
            Self::Athletics => "Athletics",
            Self::Fighting => "Fighting",
            Self::Shooting => "Shooting",
            Self::Stealth => "Stealth",
            Self::Thievery => "Thievery",
            Self::Swimming => "Swimming",
            Self::Gambling => "Gambling",
            Self::Notice => "Notice",
            Self::Survival => "Survival",
            Self::Healing => "Healing",
            Self::Repair => "Repair",
            Self::Reading => "Reading",
            Self::Persuasion => "Persuasion",
            Self::Intimidation => "Intimidation",
            Self::Climbing => "Climbing",
        }
    }

    pub fn attribute(self) -> Attribute {
        match self {
            Self::Athletics
            | Self::Fighting
            | Self::Shooting
            | Self::Stealth
            | Self::Thievery
            | Self::Swimming => Attribute::Agility,
            Self::Gambling
            | Self::Notice
            | Self::Survival
            | Self::Healing
            | Self::Repair
            | Self::Reading => Attribute::Smarts,
            Self::Persuasion | Self::Intimidation => Attribute::Spirit,
            Self::Climbing => Attribute::Strength,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Skill> {
        [
            Skill::Athletics,
            Skill::Fighting,
            Skill::Shooting,
            Skill::Stealth,
            Skill::Thievery,
            Skill::Swimming,
            Skill::Gambling,
            Skill::Notice,
            Skill::Survival,
            Skill::Healing,
            Skill::Repair,
            Skill::Reading,
            Skill::Persuasion,
            Skill::Intimidation,
            Skill::Climbing,
        ]
        .into_iter()
    }
}

impl Distribution<Skill> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Skill {
        unsafe { std::mem::transmute(rng.gen::<u8>() % Skill::iterator().count() as u8) }
    }
}
