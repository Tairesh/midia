use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::super::{
    terrains::{Dirt, DirtVariant, Pit},
    Item, Passage, Terrain, TerrainInteract, TerrainView,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Grass {
    #[serde(rename = "v")]
    variant: GrassVariant,
}

impl Grass {
    pub fn new(variant: GrassVariant) -> Self {
        Self { variant }
    }

    pub fn dead(&self) -> bool {
        matches!(
            self.variant,
            GrassVariant::DeadGrass1
                | GrassVariant::DeadGrass2
                | GrassVariant::DeadGrass3
                | GrassVariant::DeadGrass4
                | GrassVariant::DeadGrass5
                | GrassVariant::DeadGrass6
                | GrassVariant::DeadGrass7
                | GrassVariant::DeadGrass8
                | GrassVariant::DeadGrass9
                | GrassVariant::DeadGrass10
                | GrassVariant::DeadGrass11
                | GrassVariant::DeadGrass12
                | GrassVariant::DeadGrass13
                | GrassVariant::DeadGrass14
        )
    }

    pub fn die(&mut self) {
        self.variant = match self.variant {
            GrassVariant::Grass1 => GrassVariant::DeadGrass1,
            GrassVariant::Grass2 => GrassVariant::DeadGrass2,
            GrassVariant::Grass3 => GrassVariant::DeadGrass3,
            GrassVariant::Grass4 => GrassVariant::DeadGrass4,
            GrassVariant::Grass5 => GrassVariant::DeadGrass5,
            GrassVariant::Grass6 => GrassVariant::DeadGrass6,
            GrassVariant::Grass7 => GrassVariant::DeadGrass7,
            GrassVariant::Grass8 => GrassVariant::DeadGrass8,
            GrassVariant::Grass9 => GrassVariant::DeadGrass9,
            GrassVariant::Grass10 => GrassVariant::DeadGrass10,
            GrassVariant::Grass11 => GrassVariant::DeadGrass11,
            GrassVariant::Grass12 => GrassVariant::DeadGrass12,
            GrassVariant::Grass13 => GrassVariant::DeadGrass13,
            GrassVariant::Grass14 => GrassVariant::DeadGrass14,
            _ => self.variant,
        }
    }
}

impl TerrainView for Grass {
    fn name(&self) -> &str {
        if self.dead() {
            "dead grass"
        } else {
            "grass"
        }
    }

    fn looks_like(&self) -> &'static str {
        match self.variant {
            GrassVariant::Grass1 => "grass1",
            GrassVariant::Grass2 => "grass2",
            GrassVariant::Grass3 => "grass3",
            GrassVariant::Grass4 => "grass4",
            GrassVariant::Grass5 => "grass5",
            GrassVariant::Grass6 => "grass6",
            GrassVariant::Grass7 => "grass7",
            GrassVariant::Grass8 => "grass8",
            GrassVariant::Grass9 => "grass9",
            GrassVariant::Grass10 => "grass10",
            GrassVariant::Grass11 => "grass11",
            GrassVariant::Grass12 => "grass12",
            GrassVariant::Grass13 => "grass13",
            GrassVariant::Grass14 => "grass14",
            GrassVariant::DeadGrass1 => "dead_grass1",
            GrassVariant::DeadGrass2 => "dead_grass2",
            GrassVariant::DeadGrass3 => "dead_grass3",
            GrassVariant::DeadGrass4 => "dead_grass4",
            GrassVariant::DeadGrass5 => "dead_grass5",
            GrassVariant::DeadGrass6 => "dead_grass6",
            GrassVariant::DeadGrass7 => "dead_grass7",
            GrassVariant::DeadGrass8 => "dead_grass8",
            GrassVariant::DeadGrass9 => "dead_grass9",
            GrassVariant::DeadGrass10 => "dead_grass10",
            GrassVariant::DeadGrass11 => "dead_grass11",
            GrassVariant::DeadGrass12 => "dead_grass12",
            GrassVariant::DeadGrass13 => "dead_grass13",
            GrassVariant::DeadGrass14 => "dead_grass14",
        }
    }

    fn is_transparent(&self) -> bool {
        true
    }
}

impl TerrainInteract for Grass {
    fn passage(&self) -> Passage {
        Passage::Passable(11.0)
    }

    fn is_diggable(&self) -> bool {
        true
    }

    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        (Pit::new().into(), vec![])
    }

    fn can_stock_items(&self) -> bool {
        true
    }

    fn on_step(&self) -> Option<Terrain> {
        if rand::thread_rng().gen_bool(0.1) {
            Some(Dirt::new(rand::random::<DirtVariant>()).into())
        } else {
            None
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum GrassVariant {
    #[serde(rename = "1")]
    Grass1,
    #[serde(rename = "2")]
    Grass2,
    #[serde(rename = "3")]
    Grass3,
    #[serde(rename = "4")]
    Grass4,
    #[serde(rename = "5")]
    Grass5,
    #[serde(rename = "6")]
    Grass6,
    #[serde(rename = "7")]
    Grass7,
    #[serde(rename = "8")]
    Grass8,
    #[serde(rename = "9")]
    Grass9,
    #[serde(rename = "10")]
    Grass10,
    #[serde(rename = "11")]
    Grass11,
    #[serde(rename = "12")]
    Grass12,
    #[serde(rename = "13")]
    Grass13,
    #[serde(rename = "14")]
    Grass14,
    #[serde(rename = "d1")]
    DeadGrass1,
    #[serde(rename = "d2")]
    DeadGrass2,
    #[serde(rename = "d3")]
    DeadGrass3,
    #[serde(rename = "d4")]
    DeadGrass4,
    #[serde(rename = "d5")]
    DeadGrass5,
    #[serde(rename = "d6")]
    DeadGrass6,
    #[serde(rename = "d7")]
    DeadGrass7,
    #[serde(rename = "d8")]
    DeadGrass8,
    #[serde(rename = "d9")]
    DeadGrass9,
    #[serde(rename = "d10")]
    DeadGrass10,
    #[serde(rename = "d11")]
    DeadGrass11,
    #[serde(rename = "d12")]
    DeadGrass12,
    #[serde(rename = "d13")]
    DeadGrass13,
    #[serde(rename = "d14")]
    DeadGrass14,
}

impl Distribution<GrassVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GrassVariant {
        if rng.gen_bool(0.9) {
            match rng.gen_range(0..14) {
                0 => GrassVariant::Grass1,
                1 => GrassVariant::Grass2,
                2 => GrassVariant::Grass3,
                3 => GrassVariant::Grass4,
                4 => GrassVariant::Grass5,
                5 => GrassVariant::Grass6,
                6 => GrassVariant::Grass7,
                7 => GrassVariant::Grass8,
                8 => GrassVariant::Grass9,
                9 => GrassVariant::Grass10,
                10 => GrassVariant::Grass11,
                11 => GrassVariant::Grass12,
                12 => GrassVariant::Grass13,
                13 => GrassVariant::Grass14,
                _ => unreachable!(),
            }
        } else {
            match rng.gen_range(0..14) {
                0 => GrassVariant::DeadGrass1,
                1 => GrassVariant::DeadGrass2,
                2 => GrassVariant::DeadGrass3,
                3 => GrassVariant::DeadGrass4,
                4 => GrassVariant::DeadGrass5,
                5 => GrassVariant::DeadGrass6,
                6 => GrassVariant::DeadGrass7,
                7 => GrassVariant::DeadGrass8,
                8 => GrassVariant::DeadGrass9,
                9 => GrassVariant::DeadGrass10,
                10 => GrassVariant::DeadGrass11,
                11 => GrassVariant::DeadGrass12,
                12 => GrassVariant::DeadGrass13,
                13 => GrassVariant::DeadGrass14,
                _ => unreachable!(),
            }
        }
    }
}
