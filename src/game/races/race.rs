use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::traits::Name;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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
    Bug,
}

impl From<Race> for &str {
    fn from(value: Race) -> Self {
        match value {
            Race::Gazan => "gazan",
            Race::Nyarnik => "nyarnik",
            Race::Totik => "totik",
            Race::Lagnam => "lagnam",
            Race::Bug => "Bug",
        }
    }
}

impl Name for Race {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

#[derive(Sequence, Debug, Copy, Clone)]
pub enum PlayableRace {
    Gazan,
    Nyarnik,
    Totik,
    Lagnam,
}

impl PlayableRace {
    pub fn next(self) -> Self {
        next_cycle(&self).unwrap()
    }

    pub fn prev(self) -> Self {
        previous_cycle(&self).unwrap()
    }
}

impl From<PlayableRace> for &str {
    fn from(value: PlayableRace) -> Self {
        match value {
            PlayableRace::Gazan => "Gazan",
            PlayableRace::Nyarnik => "Nyarnik",
            PlayableRace::Totik => "Totik",
            PlayableRace::Lagnam => "Lagnam",
        }
    }
}

impl From<PlayableRace> for Race {
    fn from(value: PlayableRace) -> Self {
        match value {
            PlayableRace::Gazan => Race::Gazan,
            PlayableRace::Nyarnik => Race::Nyarnik,
            PlayableRace::Totik => Race::Totik,
            PlayableRace::Lagnam => Race::Lagnam,
        }
    }
}

impl From<Race> for PlayableRace {
    fn from(value: Race) -> Self {
        match value {
            Race::Gazan => PlayableRace::Gazan,
            Race::Nyarnik => PlayableRace::Nyarnik,
            Race::Totik => PlayableRace::Totik,
            Race::Lagnam => PlayableRace::Lagnam,
            Race::Bug => unreachable!(),
        }
    }
}

impl Distribution<PlayableRace> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlayableRace {
        match rng.gen_range(0..4) {
            0 => PlayableRace::Gazan,
            1 => PlayableRace::Nyarnik,
            2 => PlayableRace::Totik,
            3 => PlayableRace::Lagnam,
            _ => unreachable!(),
        }
    }
}

impl Name for PlayableRace {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}
