use super::SkillLevel;

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
