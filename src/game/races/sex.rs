use enum_iterator::Sequence;
use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::races::Gender;

#[derive(Serialize, Deserialize, Sequence, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Sex {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "u")]
    #[default]
    Other,
}

impl Sex {
    pub fn iter() -> impl Iterator<Item = Sex> {
        enum_iterator::all()
    }
}

impl From<&Gender> for Sex {
    fn from(value: &Gender) -> Self {
        match value {
            Gender::Male => Self::Male,
            Gender::Female => Self::Female,
            Gender::Custom(_) => Self::Other,
        }
    }
}

impl From<Sex> for Gender {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => Gender::Male,
            Sex::Female => Gender::Female,
            Sex::Other => Gender::Custom("None".to_string()),
        }
    }
}

impl Distribution<Sex> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sex {
        match rng.random_range(0..2) {
            0 => Sex::Male,
            1 => Sex::Female,
            _ => unreachable!(),
        }
    }
}
