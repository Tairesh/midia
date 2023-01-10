use rand::distributions::Standard;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{
    super::{bodies::BodySize, GameData},
    Gender, HairColor, MainHand, SkinTone,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appearance {
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "s")]
    pub skin_tone: SkinTone,
    #[serde(rename = "h")]
    pub hair_color: HairColor,
    #[serde(rename = "z")]
    pub body_size: BodySize,
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
        Personality::new(
            Appearance {
                age: rng.gen_range(0..=99),
                skin_tone: rng.sample(Standard),
                hair_color: rng.sample(Standard),
                body_size: rng.sample(Standard),
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
    pub fn age_name(&self) -> &str {
        age_name(self.appearance.age, Some(&self.mind.gender))
    }
}

pub fn age_name(age: u8, gender: Option<&Gender>) -> &'static str {
    match age {
        0..=3 => "baby",
        4..=15 => {
            if let Some(gender) = gender {
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
                    Gender::Male => "man",
                    Gender::Female => "woman",
                    Gender::Custom(_) => "human",
                }
            } else {
                "human"
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Appearance, BodySize, Gender, HairColor, MainHand, Mind, Personality, SkinTone};

    pub fn dead_boy() -> Personality {
        Personality::new(
            Appearance {
                age: 9,
                skin_tone: SkinTone::Almond,
                hair_color: HairColor::Black,
                body_size: BodySize::Tiny,
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
                age: 15,
                skin_tone: SkinTone::WarmIvory,
                hair_color: HairColor::Ginger,
                body_size: BodySize::Small,
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
                age: 75,
                skin_tone: SkinTone::Almond,
                hair_color: HairColor::Black,
                body_size: BodySize::Large,
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
        assert_eq!("girl", character.age_name());
    }
}
