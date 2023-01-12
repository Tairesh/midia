use rand::distributions::Standard;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::bodies::Sex;
use crate::game::races::PlayableRace;
use crate::game::traits::Name;

use super::{
    super::{bodies::BodySize, GameData},
    FurColor, Gender, MainHand, Race,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appearance {
    #[serde(rename = "r")]
    pub race: Race,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "f")]
    pub fur_color: Option<FurColor>,
    #[serde(rename = "z")]
    pub body_size: BodySize,
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
    #[serde(rename = "a")]
    pub appearance: Appearance,
    #[serde(rename = "m")]
    pub mind: Mind,
}

impl Personality {
    pub fn new(appearance: Appearance, mind: Mind) -> Self {
        Self { appearance, mind }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R, alive: bool) -> Personality {
        let gender = rng.sample(Standard);
        let game_data = GameData::instance();
        let name = format!(
            "{} {}",
            match gender {
                Gender::Male => game_data.names.random_male_name(rng),
                Gender::Female => game_data.names.random_female_name(rng),
                Gender::Custom(_) => game_data.names.random_name(rng),
            },
            game_data.names.random_name(rng)
        );
        let race: PlayableRace = rng.sample(Standard);
        let race = Race::from(race);
        Personality::new(
            Appearance {
                age: rng.gen_range(0..=99),
                fur_color: if race.has_fur() {
                    Some(rng.sample(Standard))
                } else {
                    None
                },
                body_size: rng.sample(Standard),
                sex: rng.sample(Standard),
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
        age_name(
            self.appearance.race,
            self.appearance.age,
            Some(self.mind.gender.clone()),
        )
    }
}

pub fn age_name(race: Race, age: u8, gender: Option<Gender>) -> String {
    let race_name = race.name().to_lowercase();
    match age {
        0..=3 => format!("baby {race_name}"),
        4..=15 => {
            race_name
                + " "
                + if let Some(gender) = gender {
                    match gender {
                        Gender::Male => "boy",
                        Gender::Female => "girl",
                        Gender::Custom(_) => "child",
                    }
                } else {
                    "child"
                }
        }
        16.. => {
            if let Some(gender) = gender {
                match gender {
                    Gender::Male => race_name + " man",
                    Gender::Female => race_name + " woman",
                    Gender::Custom(_) => race_name,
                }
            } else {
                race_name
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::game::bodies::Sex;

    use super::{Appearance, BodySize, FurColor, Gender, MainHand, Mind, Personality, Race};

    pub fn dead_boy() -> Personality {
        Personality::new(
            Appearance {
                race: Race::Gazan,
                age: 9,
                fur_color: Some(FurColor::Ginger),
                body_size: BodySize::Tiny,
                sex: Sex::Male,
            },
            Mind {
                name: "Dead Boy".to_string(),
                gender: Gender::Male,
                main_hand: MainHand::Right,
                alive: false,
            },
        )
    }

    pub fn tester_girl() -> Personality {
        Personality::new(
            Appearance {
                race: Race::Gazan,
                age: 15,
                fur_color: Some(FurColor::Ginger),
                body_size: BodySize::Small,
                sex: Sex::Female,
            },
            Mind {
                name: "Tester Girl".to_string(),
                gender: Gender::Female,
                main_hand: MainHand::Left,
                alive: true,
            },
        )
    }

    pub fn old_queer() -> Personality {
        Personality::new(
            Appearance {
                race: Race::Bug,
                age: 99,
                fur_color: None,
                body_size: BodySize::Large,
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
