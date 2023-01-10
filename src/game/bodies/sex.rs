use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use super::super::human::Gender;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sex {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
}

impl Default for Sex {
    fn default() -> Self {
        Self::Female
    }
}

impl TryFrom<&Gender> for Sex {
    type Error = &'static str;

    fn try_from(value: &Gender) -> Result<Self, Self::Error> {
        match value {
            Gender::Male => Ok(Self::Male),
            Gender::Female => Ok(Self::Female),
            Gender::Custom(_) => Err("Can't match custom genders to biological sex"),
        }
    }
}

impl From<Sex> for Gender {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => Gender::Male,
            Sex::Female => Gender::Female,
        }
    }
}
