use std::collections::{HashMap, HashSet};

use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::assets::Sprite;
use crate::colors::Colors;
use crate::game::{
    savage::{DamageDice, DamageType, Skill},
    traits::{LooksLike, Name},
    units::{Inventory, Weapon},
    AttackType, Attribute, Damage, DamageValue, SkillLevel,
};

use super::BodyColor;

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
    pub fn is_intelligent(self) -> bool {
        match self {
            Self::Gazan | Self::Lagnam | Self::Totik | Self::Nyarnik => true,
            Self::Bug => false,
        }
    }

    pub fn has_custom_colors(self) -> bool {
        !self.custom_colors().is_empty()
    }

    pub fn custom_colors(self) -> Vec<BodyColor> {
        match self {
            Self::Gazan => vec![
                BodyColor::LightBrown,
                BodyColor::Ginger,
                BodyColor::DarkBrown,
                BodyColor::LightGreen,
                BodyColor::Green,
                BodyColor::DarkGreen,
                BodyColor::White,
                BodyColor::Gray,
                BodyColor::DarkGray,
                BodyColor::Albino,
            ],
            Self::Lagnam => vec![
                BodyColor::LightBrown,
                BodyColor::Ginger,
                BodyColor::DarkBrown,
                BodyColor::White,
                BodyColor::Gray,
                BodyColor::DarkGray,
                BodyColor::Albino,
            ],
            Self::Totik => vec![
                BodyColor::LightBlue,
                BodyColor::Blue,
                BodyColor::DarkBlue,
                BodyColor::GreenBlue,
                BodyColor::LightGreen,
                BodyColor::Green,
                BodyColor::DarkGreen,
                BodyColor::Albino,
            ],
            Self::Nyarnik => vec![
                BodyColor::LightBrown,
                BodyColor::Ginger,
                BodyColor::DarkBrown,
                BodyColor::OrangeRed,
                BodyColor::Albino,
            ],
            Self::Bug => vec![BodyColor::Lime, BodyColor::Red],
        }
    }

    pub fn free_skills(self) -> HashMap<Skill, SkillLevel> {
        HashMap::from_iter(match self {
            Race::Gazan => vec![(Skill::Climbing, SkillLevel::D6)],
            Race::Totik => vec![(Skill::Swimming, SkillLevel::D6)],
            _ => vec![],
        })
    }

    pub fn natural_armor(self) -> i8 {
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

    // TODO: use enum_iterator
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

    pub fn natural_weapon(self) -> Weapon {
        // TODO: consts for names
        match self {
            Race::Gazan | Race::Nyarnik => Weapon {
                name: "fists".to_string(),
                damage: DamageValue::strength(DamageType::Blunt),
            },
            Race::Totik | Race::Lagnam => Weapon {
                name: "fangs".to_string(),
                damage: DamageValue::simple(DamageDice::D4, DamageType::Pierce),
            },
            Race::Bug => Weapon {
                name: "mandibles".to_string(),
                // TODO: poison
                damage: DamageValue::simple(DamageDice::D6, DamageType::Pierce),
            },
        }
    }

    pub fn inventory(self) -> Inventory {
        match self {
            Race::Gazan | Race::Nyarnik | Race::Totik | Race::Lagnam => Inventory::humanoid(),
            Race::Bug => Inventory::monster(),
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
            Race::Bug => "giant bug",
        }
    }
}

impl Name for Race {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

impl LooksLike for Race {
    fn looks_like(&self) -> Sprite {
        match self {
            Race::Gazan => Sprite::Gazan,
            Race::Nyarnik => Sprite::Nyarnik,
            Race::Totik => Sprite::Totik,
            Race::Lagnam => Sprite::Lagnam,
            Race::Bug => Sprite::GiantBug,
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
