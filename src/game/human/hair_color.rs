use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum HairColor {
    #[serde(rename = "1")]
    White, // Only for unnatural beings
    #[serde(rename = "2")]
    Gray, // For oldies
    #[serde(rename = "3")]
    Blonde,
    #[serde(rename = "4")]
    Ginger,
    #[serde(rename = "5")]
    LightBrown,
    #[serde(rename = "6")]
    MediumBrown,
    #[serde(rename = "7")]
    DarkBrown,
    #[serde(rename = "8")]
    Black,
}

impl Distribution<HairColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HairColor {
        match rng.gen_range(0..6) {
            0 => HairColor::Blonde,
            1 => HairColor::Ginger,
            2 => HairColor::LightBrown,
            3 => HairColor::MediumBrown,
            4 => HairColor::DarkBrown,
            5 => HairColor::Black,
            _ => unreachable!(),
        }
    }
}
