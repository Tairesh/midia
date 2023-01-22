use std::collections::{HashMap, HashSet};

use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::game_data::DamageType;
use crate::game::savage::{DamageDice, Skill};
use crate::game::traits::Name;
use crate::game::{Attribute, Damage, MeleeDamageValue, SkillLevel};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Race {
    Gazan,
    Nyarnik,
    Totik,
    Lagnam,
    Bug,
}

impl Race {
    pub fn has_fur(self) -> bool {
        matches!(self, Race::Gazan | Race::Lagnam)
    }

    pub fn free_skills(self) -> HashMap<Skill, SkillLevel> {
        HashMap::from_iter(match self {
            Race::Gazan => vec![(Skill::Climbing, SkillLevel::D6)],
            Race::Totik => vec![(Skill::Swimming, SkillLevel::D6)],
            _ => vec![],
        })
    }

    pub fn toughness_bonus(self) -> i8 {
        match self {
            Race::Nyarnik => 2,
            _ => 0,
        }
    }

    pub fn walk_koeff(self) -> f32 {
        match self {
            Race::Nyarnik => 1.2,
            Race::Lagnam => 0.8,
            _ => 1.0,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Race> {
        [
            Self::Gazan,
            Self::Lagnam,
            Self::Nyarnik,
            Self::Totik,
            Self::Bug,
        ]
        .iter()
        .copied()
    }

    pub fn natural_weapon(self) -> (&'static str, MeleeDamageValue) {
        match self {
            Race::Gazan | Race::Nyarnik => (
                "fists",
                MeleeDamageValue {
                    damage: Damage {
                        dices: Vec::new(),
                        attribute: Some(Attribute::Strength),
                        modifier: 0,
                    },
                    damage_types: HashSet::from([DamageType::Blunt]),
                    distance: 0,
                    penetration: 0,
                },
            ),
            Race::Totik | Race::Lagnam => (
                "fangs",
                MeleeDamageValue {
                    damage: Damage {
                        dices: vec![DamageDice::D4],
                        attribute: Some(Attribute::Strength),
                        modifier: 0,
                    },
                    damage_types: HashSet::from([DamageType::Pierce]),
                    distance: 0,
                    penetration: 0,
                },
            ),
            Race::Bug => (
                "mandibles",
                MeleeDamageValue {
                    damage: Damage {
                        dices: vec![DamageDice::D6],
                        attribute: Some(Attribute::Strength),
                        modifier: 0,
                    },
                    damage_types: HashSet::from([DamageType::Pierce]),
                    distance: 0,
                    penetration: 0,
                },
            ),
        }
    }
}

impl From<Race> for &str {
    fn from(value: Race) -> Self {
        match value {
            Race::Gazan => "gazan",
            Race::Nyarnik => "nyarnik",
            Race::Totik => "totik",
            Race::Lagnam => "lagnam",
            Race::Bug => "bug",
        }
    }
}

impl Name for Race {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..5) {
            0 => Race::Gazan,
            1 => Race::Nyarnik,
            2 => Race::Totik,
            3 => Race::Lagnam,
            4 => Race::Bug,
            _ => unreachable!(),
        }
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
