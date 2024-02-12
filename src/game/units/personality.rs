use rand::{distributions::Standard, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use super::super::{
    races::{BodyColor, Gender, PlayableRace, Race, Sex},
    traits::Name,
    CharSheet, GameData,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appearance {
    #[serde(rename = "r")]
    pub race: Race,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "c")]
    pub body_color: Option<BodyColor>,
    // TODO: sexes should be defined by race
    #[serde(rename = "x")]
    pub sex: Sex,
}

impl Appearance {
    pub fn body_name(&self) -> String {
        // TODO: race-specific sexes and ages
        let race_name = self.race.name().to_lowercase();
        if self.race.is_intelligent() {
            match self.age {
                0..=3 => format!("baby {race_name}"),
                4..=15 => {
                    race_name
                        + " "
                        + match self.sex {
                            Sex::Male => "boy",
                            Sex::Female => "girl",
                            Sex::Undefined => "child",
                        }
                }
                16.. => {
                    race_name
                        + match self.sex {
                            Sex::Male => " man",
                            Sex::Female => " woman",
                            Sex::Undefined => "",
                        }
                }
            }
        } else {
            match self.age {
                0..=5 => format!("young {race_name}"),
                _ => match self.sex {
                    Sex::Female => format!("{race_name} queen"),
                    _ => race_name,
                },
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mind {
    #[serde(rename = "n")]
    pub name: String,
    // TODO: remove
    #[serde(rename = "g")]
    pub gender: Gender,
    // TODO: profession
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerPersonality {
    #[serde(rename = "a")]
    pub appearance: Appearance,
    #[serde(rename = "m")]
    pub mind: Mind,
    #[serde(rename = "c")]
    pub char_sheet: CharSheet,
}

impl PlayerPersonality {
    pub fn new(appearance: Appearance, mind: Mind, char_sheet: CharSheet) -> Self {
        Self {
            appearance,
            mind,
            char_sheet,
        }
    }

    pub fn random_playable<R: Rng + ?Sized>(rng: &mut R) -> PlayerPersonality {
        let gender = rng.sample(Standard);
        let sex = Sex::from(&gender);
        let game_data = GameData::instance();
        let race: PlayableRace = rng.sample(Standard);
        let race = Race::from(race);
        let age = rng.gen_range(0..=99);
        let name = game_data
            .names
            .get(&race)
            .unwrap()
            .get(&sex)
            .unwrap()
            .choose(rng)
            .cloned()
            .unwrap_or_default();
        PlayerPersonality::new(
            Appearance {
                body_color: if race.custom_colors().is_empty() {
                    None
                } else {
                    Some(*race.custom_colors().choose(rng).unwrap())
                },
                age,
                sex,
                race,
            },
            Mind { name, gender },
            CharSheet::default(true, race),
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::game::savage::{AttrLevel, Attributes, Skills};
    use crate::game::{Dice, SkillLevel};

    use super::{Appearance, BodyColor, CharSheet, Gender, Mind, PlayerPersonality, Race, Sex};

    pub fn tester_girl() -> PlayerPersonality {
        PlayerPersonality::new(
            Appearance {
                race: Race::Gazan,
                age: 15,
                body_color: Some(BodyColor::Ginger),
                sex: Sex::Female,
            },
            Mind {
                name: "Dooka".to_string(),
                gender: Gender::Female,
            },
            CharSheet::default(true, Race::Gazan),
        )
    }

    pub fn shasha() -> PlayerPersonality {
        PlayerPersonality::new(
            Appearance {
                race: Race::Nyarnik,
                age: 22,
                body_color: Some(BodyColor::LightGreen),
                sex: Sex::Female,
            },
            Mind {
                name: "Shasha".to_string(),
                gender: Gender::Female,
            },
            CharSheet::new(
                true,
                Race::Nyarnik,
                Attributes {
                    agility: AttrLevel::D8,
                    smarts: AttrLevel::D6,
                    spirit: AttrLevel::D6,
                    strength: AttrLevel::D10,
                    vigor: AttrLevel::D6,
                },
                Skills {
                    athletics: SkillLevel::None,
                    fighting: SkillLevel::D12,
                    shooting: SkillLevel::None,
                    stealth: SkillLevel::None,
                    thievery: SkillLevel::None,
                    swimming: SkillLevel::None,
                    gambling: SkillLevel::D6,
                    notice: SkillLevel::None,
                    survival: SkillLevel::None,
                    healing: SkillLevel::None,
                    repair: SkillLevel::None,
                    reading: SkillLevel::None,
                    persuasion: SkillLevel::D8,
                    intimidation: SkillLevel::None,
                    climbing: SkillLevel::None,
                    // TODO: streetwise
                    // TODO: cooking
                    // TODO: taunt
                },
            ),
        )
    }

    #[test]
    fn test_body_name() {
        let character = tester_girl();
        assert_eq!("gazan girl", character.appearance.body_name());
    }
}
