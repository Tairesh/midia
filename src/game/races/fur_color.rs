use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::colors::Colors;
use crate::game::traits::Name;

#[derive(Serialize, Deserialize, Sequence, Debug, Copy, Clone, Eq, PartialEq)]
pub enum FurColor {
    #[serde(rename = "1")]
    Albino,
    #[serde(rename = "2")]
    White,
    #[serde(rename = "3")]
    LightBrown,
    #[serde(rename = "4")]
    DarkBrown,
    #[serde(rename = "5")]
    Ginger,
    #[serde(rename = "6")]
    Gray,
}

impl FurColor {
    pub fn text_color(self) -> Color {
        match self {
            FurColor::Albino | FurColor::White | FurColor::LightBrown => Colors::BLACK,
            _ => Colors::WHITE_SMOKE,
        }
    }
    pub fn next(self) -> Self {
        next_cycle(&self).unwrap()
    }

    pub fn prev(self) -> Self {
        previous_cycle(&self).unwrap()
    }
}

impl Distribution<FurColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FurColor {
        match rng.gen_range(0..6) {
            0 => FurColor::Albino,
            1 => FurColor::Ginger,
            2 => FurColor::LightBrown,
            3 => FurColor::DarkBrown,
            4 => FurColor::White,
            5 => FurColor::Gray,
            _ => unreachable!(),
        }
    }
}

impl From<FurColor> for Color {
    fn from(s: FurColor) -> Self {
        match s {
            FurColor::Albino => Colors::WHITE_SMOKE,
            FurColor::Gray => Colors::GRAY,
            FurColor::White => Colors::BLANCHED_ALMOND,
            FurColor::Ginger => Colors::ORANGE,
            FurColor::LightBrown => Colors::SANDY_BROWN,
            FurColor::DarkBrown => Colors::SADDLE_BROWN,
        }
    }
}

impl From<FurColor> for &str {
    fn from(value: FurColor) -> Self {
        match value {
            FurColor::Albino => "Albino",
            FurColor::Gray => "Gray",
            FurColor::White => "White",
            FurColor::Ginger => "Ginger",
            FurColor::LightBrown => "Light Brown",
            FurColor::DarkBrown => "Dark Brown",
        }
    }
}

impl Name for FurColor {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}
