use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::pronouns::Pronouns;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Gender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "x")]
    Custom(String),
}

impl Gender {
    pub fn pronouns(&self) -> Pronouns {
        match self {
            Gender::Male => Pronouns::HeHim,
            Gender::Female => Pronouns::SheHer,
            Gender::Custom(_) => Pronouns::TheyThem,
        }
    }
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Custom(value),
        }
    }
}

impl From<Gender> for String {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Male => "Male".to_string(),
            Gender::Female => "Female".to_string(),
            Gender::Custom(s) => s,
        }
    }
}

impl Distribution<Gender> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        if rng.random_bool(0.51) {
            Gender::Female
        } else {
            Gender::Male
        }
    }
}
