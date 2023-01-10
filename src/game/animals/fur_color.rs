use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum FurColor {
    #[serde(rename = "1")]
    White,
    #[serde(rename = "2")]
    Gray,
    #[serde(rename = "3")]
    Yellow,
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
