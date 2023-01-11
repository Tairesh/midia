use rand::distributions::{Distribution, Standard};
use rand::Rng;

use super::{
    super::races::{FurColor, Personality, Race, SkinTone},
    Sex,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum Freshness {
    #[serde(rename = "f")]
    Fresh,
    #[serde(rename = "r")]
    Rotten,
    #[serde(rename = "s")]
    Skeletal,
}

impl Freshness {
    pub fn adjective(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Rotten => "rotten",
            Self::Skeletal => "skeletal",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum BodySize {
    #[serde(rename = "1")]
    Tiny,
    #[serde(rename = "2")]
    Small,
    #[serde(rename = "3")]
    Normal,
    #[serde(rename = "4")]
    Large,
    #[serde(rename = "5")]
    Huge,
}

impl Distribution<BodySize> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BodySize {
        match rng.gen_range(0..5) {
            // TODO: normal distribution
            0 => BodySize::Tiny,
            1 => BodySize::Small,
            2 => BodySize::Normal,
            3 => BodySize::Large,
            4 => BodySize::Huge,
            _ => unreachable!(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct OrganData {
    #[serde(rename = "r")]
    pub race: Race,
    #[serde(rename = "f")]
    pub freshness: Freshness,
    #[serde(rename = "a")]
    pub age: u8,
    #[serde(rename = "s")]
    pub size: BodySize,
    #[serde(rename = "l")]
    pub alive: bool,
    #[serde(rename = "x")]
    pub sex: Sex,
    #[serde(rename = "t")]
    pub skin_tone: SkinTone,
    #[serde(rename = "c")]
    pub fur_color: Option<FurColor>,
}

// TODO: SkinData with SkinTone and scars/tattoo/etc.

impl OrganData {
    pub fn new(character: &Personality, freshness: Freshness) -> Self {
        Self {
            freshness,
            race: character.appearance.race,
            age: character.appearance.age,
            size: character.appearance.body_size,
            alive: character.mind.alive,
            sex: character.appearance.sex,
            skin_tone: character.appearance.skin_tone,
            fur_color: character.appearance.fur_color,
        }
    }
}
