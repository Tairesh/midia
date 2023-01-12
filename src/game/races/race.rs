use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Sequence, Debug, Copy, Clone)]
pub enum Race {
    #[serde(rename = "g")]
    Gazan,
    #[serde(rename = "n")]
    Nyarnik,
    #[serde(rename = "t")]
    Totik,
    #[serde(rename = "l")]
    Lagnam,
    #[serde(rename = "b")]
    Bug, // TODO: create enum for only playable races
}

impl Race {
    pub fn name(self) -> &'static str {
        self.into()
    }

    pub fn next(self) -> Self {
        next_cycle(&self).unwrap()
    }

    pub fn prev(self) -> Self {
        previous_cycle(&self).unwrap()
    }
}

impl From<Race> for &str {
    fn from(value: Race) -> Self {
        match value {
            Race::Gazan => "Gazan",
            Race::Nyarnik => "Nyarnik",
            Race::Totik => "Totik",
            Race::Lagnam => "Lagnam",
            Race::Bug => "Bug",
        }
    }
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..4) {
            0 => Race::Gazan,
            1 => Race::Nyarnik,
            2 => Race::Totik,
            3 => Race::Lagnam,
            _ => unreachable!(),
        }
    }
}
