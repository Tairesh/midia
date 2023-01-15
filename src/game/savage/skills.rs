use crate::game::races::Race;

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
            athletics: SkillLevel::D4_2,
            fighting: SkillLevel::D4_2,
            shooting: SkillLevel::D4_2,
            stealth: SkillLevel::D4_2,
            thievery: SkillLevel::D4_2,
            swimming: if race == Race::Totik {
                SkillLevel::D6
            } else {
                SkillLevel::D4_2
            },
            gambling: SkillLevel::D4_2,
            notice: SkillLevel::D4_2,
            survival: SkillLevel::D4_2,
            healing: SkillLevel::D4_2,
            repair: SkillLevel::D4_2,
            reading: SkillLevel::D4_2,
            persuasion: SkillLevel::D4_2,
            intimidation: SkillLevel::D4_2,
            climbing: if race == Race::Gazan {
                SkillLevel::D6
            } else {
                SkillLevel::D4_2
            },
        }
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

    #[allow(dead_code)]
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
