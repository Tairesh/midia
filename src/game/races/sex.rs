use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::races::Gender;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Sex {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "u")]
    Undefined,
}

impl Sex {
    pub fn iterator() -> impl Iterator<Item = Sex> {
        [Self::Male, Self::Female, Self::Undefined].iter().copied()
    }
}

impl Default for Sex {
    fn default() -> Self {
        Self::Female
    }
}

impl From<&Gender> for Sex {
    fn from(value: &Gender) -> Self {
        match value {
            Gender::Male => Self::Male,
            Gender::Female => Self::Female,
            Gender::Custom(_) => Self::Undefined,
        }
    }
}

impl From<Sex> for Gender {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => Gender::Male,
            Sex::Female => Gender::Female,
            Sex::Undefined => Gender::Custom("None".to_string()),
        }
    }
}

impl Distribution<Sex> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sex {
        match rng.gen_range(0..2) {
            0 => Sex::Male,
            1 => Sex::Female,
            _ => unreachable!(),
        }
    }
}
