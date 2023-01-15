use rand::distributions::Standard;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{
    super::{traits::Name, GameData},
    FurColor, Gender, MainHand, PlayableRace, Race, Sex,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appearance {
    #[serde(rename = "r")]
    pub race: Race,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "f")]
    pub fur_color: Option<FurColor>,
    #[serde(rename = "x")]
    pub sex: Sex,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mind {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "g")]
    pub gender: Gender,
    #[serde(rename = "m")]
    pub main_hand: MainHand,
    #[serde(rename = "l")]
    pub alive: bool,
    // TODO: profession
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Personality {
    #[serde(rename = "p")]
    pub is_player: bool,
    #[serde(rename = "a")]
    pub appearance: Appearance,
    #[serde(rename = "m")]
    pub mind: Mind,
}

impl Personality {
    pub fn new(is_player: bool, appearance: Appearance, mind: Mind) -> Self {
        Self {
            is_player,
            appearance,
            mind,
        }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R, is_player: bool, alive: bool) -> Personality {
        let gender = rng.sample(Standard);
        let sex = Sex::from(&gender);
        let game_data = GameData::instance();
        let race = if is_player {
            let race: PlayableRace = rng.sample(Standard);
            Race::from(race)
        } else {
            rng.sample(Standard)
        };
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
            is_player,
            Appearance {
                age: rng.gen_range(0..=99),
                fur_color: if race.has_fur() {
                    Some(rng.sample(Standard))
                } else {
                    None
                },
                sex,
                race,
            },
            Mind {
                name,
                gender,
                main_hand: rng.sample(Standard),
                alive,
            },
        )
    }

    #[allow(dead_code)]
    pub fn age_name(&self) -> String {
        age_name(&self.appearance)
    }
}

pub fn age_name(appearance: &Appearance) -> String {
    let race_name = appearance.race.name().to_lowercase();
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
                + " "
                + match appearance.sex {
                    Sex::Male => "man",
                    Sex::Female => "woman",
                    Sex::Undefined => "person",
                }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Appearance, FurColor, Gender, MainHand, Mind, Personality, Race, Sex};

    pub fn tester_girl() -> Personality {
        Personality::new(
            false,
            Appearance {
                race: Race::Gazan,
                age: 15,
                fur_color: Some(FurColor::Ginger),
                sex: Sex::Female,
            },
            Mind {
                name: "Dooka".to_string(),
                gender: Gender::Female,
                main_hand: MainHand::Left,
                alive: true,
            },
        )
    }

    pub fn old_queer() -> Personality {
        Personality::new(
            false,
            Appearance {
                race: Race::Bug,
                age: 99,
                fur_color: None,
                sex: Sex::Undefined,
            },
            Mind {
                name: "Old Queer".to_string(),
                gender: Gender::Custom("X".to_string()),
                main_hand: MainHand::Ambidexter,
                alive: true,
            },
        )
    }

    #[test]
    fn test_age_name() {
        let character = tester_girl();
        assert_eq!("gazan girl", character.age_name());
    }
}
