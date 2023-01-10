use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::super::{
    super::{
        bodies::Freshness,
        human::{helpers::human_body, Personality},
    },
    items::{Corpse, Gravestone, Rags},
    terrains::Pit,
    Item, Passage, Terrain, TerrainInteract, TerrainView,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Grave {
    #[serde(rename = "v")]
    variant: GraveVariant,
    #[serde(rename = "d")]
    data: GraveData,
}

impl Grave {
    pub fn new(variant: GraveVariant, data: GraveData) -> Self {
        Self { variant, data }
    }
}

impl TerrainView for Grave {
    fn name(&self) -> &str {
        match self.variant {
            GraveVariant::New => "grave",
            GraveVariant::Old => "old grave",
        }
    }

    fn looks_like(&self) -> &'static str {
        match self.variant {
            GraveVariant::New => "grave_new",
            GraveVariant::Old => "grave_old",
        }
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl TerrainInteract for Grave {
    fn passage(&self) -> Passage {
        Passage::Impassable
    }

    fn is_diggable(&self) -> bool {
        true
    }

    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        let mut body = human_body(
            &self.data.character,
            match self.data.death_year {
                253..=255 => Freshness::Rotten,
                _ => Freshness::Skeletal,
            },
        );
        body.wear.push(Rags::new().into());
        (
            Pit::new().into(),
            vec![
                Gravestone::new(self.data.clone()).into(),
                Corpse::new(self.data.character.clone(), body).into(),
            ],
        )
    }

    fn is_readable(&self) -> bool {
        true
    }

    fn read(&self) -> String {
        self.data.read()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum GraveVariant {
    New,
    Old,
}

impl Distribution<GraveVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GraveVariant {
        if rng.gen_bool(0.9) {
            GraveVariant::Old
        } else {
            GraveVariant::New
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GraveData {
    #[serde(rename = "c")]
    pub character: Personality,
    #[serde(rename = "d")]
    pub death_year: u8,
}

impl GraveData {
    pub fn read(&self) -> String {
        format!(
            "You read on gravestone: {}. {} — {}",
            self.character.mind.name, // TODO: random mottos, professions, etc.
            self.death_year as i32 - self.character.appearance.age as i32,
            self.death_year
        )
    }
}
