use rand::{distributions::Standard, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use super::{
    super::{traits::Name, CharSheet, GameData},
    BodyColor, Gender, PlayableRace, Race, Sex,
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
pub struct Personality {
    #[serde(rename = "a")]
    pub appearance: Appearance,
    #[serde(rename = "m")]
    pub mind: Mind,
    #[serde(rename = "c")]
    pub char_sheet: CharSheet,
}

impl Personality {
    pub fn new(appearance: Appearance, mind: Mind, char_sheet: CharSheet) -> Self {
        Self {
            appearance,
            mind,
            char_sheet,
        }
    }

    pub fn random_playable<R: Rng + ?Sized>(rng: &mut R) -> Personality {
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
        Personality::new(
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
            CharSheet::default(true, race, age),
        )
    }

    #[allow(dead_code)]
    pub fn age_name(&self) -> String {
        age_name(&self.appearance)
    }
}

pub fn age_name(appearance: &Appearance) -> String {
    let race_name = appearance.race.name().to_lowercase();
    if appearance.race.is_intelligent() {
        match appearance.age {
            0..=3 => format!("baby {race_name}"),
            4..=15 => {
                race_name
                    + " "
                    + match appearance.sex {
                        Sex::Male => "boy",
                        Sex::Female => "girl",
                        Sex::Undefined => "child",
                    }
            }
            16.. => {
                race_name
                    + match appearance.sex {
                        Sex::Male => " man",
                        Sex::Female => " woman",
                        Sex::Undefined => "",
                    }
            }
        }
    } else {
        match appearance.age {
            0..=5 => format!("young {race_name}"),
            _ => match appearance.sex {
                Sex::Female => format!("{race_name} queen"),
                _ => race_name,
            },
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Appearance, BodyColor, CharSheet, Gender, Mind, Personality, Race, Sex};

    pub fn tester_girl() -> Personality {
        Personality::new(
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
            CharSheet::default(true, Race::Gazan, 25),
        )
    }

    pub fn old_bugger() -> Personality {
        Personality::new(
            Appearance {
                race: Race::Bug,
                age: 99,
                body_color: None,
                sex: Sex::Undefined,
            },
            Mind {
                name: "Old Queer".to_string(),
                gender: Gender::Custom("X".to_string()),
            },
            CharSheet::default(false, Race::Bug, 99),
        )
    }

    #[test]
    fn test_age_name() {
        let character = tester_girl();
        assert_eq!("gazan girl", character.age_name());
    }
}
