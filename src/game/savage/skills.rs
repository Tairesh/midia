use super::{Attribute, SkillLevel};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
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
    #[allow(dead_code)]
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
        vec![
            (Attribute::Agility, Skill::Athletics, self.athletics),
            (Attribute::Agility, Skill::Fighting, self.fighting),
            (Attribute::Agility, Skill::Shooting, self.shooting),
            (Attribute::Agility, Skill::Stealth, self.stealth),
            (Attribute::Agility, Skill::Thievery, self.thievery),
            (Attribute::Agility, Skill::Swimming, self.swimming),
            (Attribute::Smarts, Skill::Gambling, self.gambling),
            (Attribute::Smarts, Skill::Notice, self.notice),
            (Attribute::Smarts, Skill::Survival, self.survival),
            (Attribute::Smarts, Skill::Healing, self.healing),
            (Attribute::Smarts, Skill::Repair, self.repair),
            (Attribute::Smarts, Skill::Reading, self.reading),
            (Attribute::Spirit, Skill::Persuasion, self.persuasion),
            (Attribute::Spirit, Skill::Intimidation, self.intimidation),
            (Attribute::Strength, Skill::Climbing, self.climbing),
        ]
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
