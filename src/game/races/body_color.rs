use std::fmt::{Display, Formatter};

use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::colors::Colors;
use crate::game::traits::Name;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BodyColor {
    Albino,
    White,
    LightBrown,
    DarkBrown,
    Ginger,
    Gray,
    DarkGray,
    LightGreen,
    Green,
    DarkGreen,
    LightBlue,
    Blue,
    DarkBlue,
    GreenBlue,
    OrangeRed,
    // For monsters
    Lime,
    Red,
    Violet,
}

impl BodyColor {
    pub fn text_color(self) -> Color {
        match self {
            BodyColor::Albino
            | BodyColor::White
            | BodyColor::LightBrown
            | BodyColor::LightGreen
            | BodyColor::Ginger
            | BodyColor::LightBlue => Colors::BLACK,
            _ => Colors::WHITE_SMOKE,
        }
    }
}

pub struct BugColorDistribution {}

impl Distribution<BodyColor> for BugColorDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BodyColor {
        match rng.random_range(0..3) {
            0 => BodyColor::Lime,
            1 => BodyColor::Red,
            2 => BodyColor::Violet,
            _ => unreachable!(),
        }
    }
}

impl From<BodyColor> for Color {
    fn from(s: BodyColor) -> Self {
        match s {
            BodyColor::Albino => Colors::WHITE_SMOKE,
            BodyColor::Gray => Colors::GRAY,
            BodyColor::DarkGray => Colors::DARK_GRAY,
            BodyColor::White => Colors::BLANCHED_ALMOND,
            BodyColor::Ginger => Colors::ORANGE,
            BodyColor::LightBrown => Colors::SANDY_BROWN,
            BodyColor::DarkBrown => Colors::SADDLE_BROWN,
            BodyColor::LightGreen => Colors::LIGHT_GREEN,
            BodyColor::Green => Colors::GREEN,
            BodyColor::DarkGreen => Colors::DARK_GREEN,
            BodyColor::LightBlue => Colors::LIGHT_BLUE,
            BodyColor::Blue => Colors::BLUE,
            BodyColor::DarkBlue => Colors::DARK_SLATE_BLUE,
            BodyColor::GreenBlue => Colors::SEA_GREEN,
            BodyColor::OrangeRed => Colors::ORANGE_RED,
            BodyColor::Lime => Colors::LIME_GREEN,
            BodyColor::Red => Colors::RED,
            BodyColor::Violet => Colors::VIOLET,
        }
    }
}

impl From<BodyColor> for &str {
    fn from(value: BodyColor) -> Self {
        match value {
            BodyColor::Albino => "Albino",
            BodyColor::Gray => "Gray",
            BodyColor::White => "White",
            BodyColor::Ginger => "Ginger",
            BodyColor::LightBrown => "Light Brown",
            BodyColor::DarkBrown => "Dark Brown",
            BodyColor::Green => "Green",
            BodyColor::DarkGray => "Black",
            BodyColor::LightGreen => "Light Green",
            BodyColor::DarkGreen => "Dark Green",
            BodyColor::LightBlue => "Light Blue",
            BodyColor::Blue => "Blue",
            BodyColor::DarkBlue => "Dark Blue",
            BodyColor::GreenBlue => "Green Blue",
            BodyColor::OrangeRed => "Orange Red",
            BodyColor::Lime => "Lime",
            BodyColor::Red => "Red",
            BodyColor::Violet => "Violet",
        }
    }
}

impl Name for BodyColor {
    fn name(&self) -> &'static str {
        (*self).into()
    }
}

impl Display for BodyColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub fn next_color(current: BodyColor, colors: &[BodyColor], forward: bool) -> BodyColor {
    let max_i = colors.len() - 1;
    let mut i = colors.iter().position(|&c| c == current).unwrap();
    if forward {
        if i == max_i {
            i = 0;
        } else {
            i += 1;
        }
    } else if i == 0 {
        i = max_i;
    } else {
        i -= 1;
    }

    *colors.get(i).unwrap()
}

#[cfg(test)]
pub mod tests {
    use test_case::test_case;

    use super::{next_color, BodyColor};

    #[test_case(BodyColor::Ginger, & vec ! [BodyColor::Ginger, BodyColor::LightBrown, BodyColor::Green], true, BodyColor::LightBrown)]
    #[test_case(BodyColor::Green, & vec ! [BodyColor::Ginger, BodyColor::LightBrown, BodyColor::Green], true, BodyColor::Ginger)]
    #[test_case(BodyColor::Ginger, & vec ! [BodyColor::Ginger, BodyColor::LightBrown, BodyColor::Green], false, BodyColor::Green)]
    #[test_case(BodyColor::Green, & vec ! [BodyColor::Ginger, BodyColor::LightBrown, BodyColor::Green], false, BodyColor::LightBrown)]
    fn test_next_color(
        current: BodyColor,
        colors: &Vec<BodyColor>,
        forward: bool,
        expect: BodyColor,
    ) {
        assert_eq!(next_color(current, colors, forward), expect);
    }
}
