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
    White,
    #[serde(rename = "2")]
    Yellow,
    #[serde(rename = "3")]
    LightBrown,
    #[serde(rename = "4")]
    MediumBrown,
    #[serde(rename = "5")]
    DarkBrown,
    #[serde(rename = "6")]
    Ginger,
    #[serde(rename = "7")]
    Gray,
    #[serde(rename = "8")]
    Black,
}

impl FurColor {
    pub fn text_color(self) -> Color {
        match self {
            FurColor::White | FurColor::Yellow | FurColor::LightBrown => Colors::BLACK,
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
        match rng.gen_range(0..8) {
            0 => FurColor::White,
            1 => FurColor::Ginger,
            2 => FurColor::LightBrown,
            3 => FurColor::MediumBrown,
            4 => FurColor::DarkBrown,
            5 => FurColor::Black,
            6 => FurColor::Yellow,
            7 => FurColor::Gray,
            _ => unreachable!(),
        }
    }
}

impl From<FurColor> for Color {
    fn from(s: FurColor) -> Self {
        match s {
            FurColor::White => Colors::WHITE_SMOKE,
            FurColor::Gray => Colors::DARK_GRAY,
            FurColor::Yellow => Colors::LIGHT_YELLOW,
            FurColor::Ginger => Colors::BRONZE,
            FurColor::LightBrown => Colors::SANDY_BROWN,
            FurColor::MediumBrown => Colors::SADDLE_BROWN,
            FurColor::DarkBrown => Colors::DARK_BROWN,
            FurColor::Black => Colors::DARKEST_GRAY,
        }
    }
}

impl From<FurColor> for &str {
    fn from(value: FurColor) -> Self {
        match value {
            FurColor::White => "White",
            FurColor::Gray => "Gray",
            FurColor::Yellow => "Yellow",
            FurColor::Ginger => "Ginger",
            FurColor::LightBrown => "Light Brown",
            FurColor::MediumBrown => "Medium Brown",
            FurColor::DarkBrown => "Dark Brown",
            FurColor::Black => "Black",
        }
    }
}

impl Name for FurColor {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}
