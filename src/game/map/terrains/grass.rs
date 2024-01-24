use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use tetra::graphics::Color;

use crate::colors::Colors;
use crate::game::traits::{LooksLike, Name};

use super::super::{
    terrains::{Dirt, DirtVariant, Pit},
    Item, Passage, Terrain, TerrainInteract, TerrainView,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Grass {
    #[serde(rename = "v")]
    variant: GrassVariant,
    #[serde(rename = "d")]
    dead: bool,
}

impl Grass {
    pub fn new(variant: GrassVariant, dead: bool) -> Self {
        Self { variant, dead }
    }

    pub fn die(&mut self) {
        self.dead = true;
    }
}

impl TerrainView for Grass {
    fn name(&self) -> &'static str {
        if self.dead {
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
            GrassVariant::Grass15 => "grass15",
            GrassVariant::Grass16 => "grass16",
            GrassVariant::Grass17 => "grass17",
            GrassVariant::Grass18 => "grass18",
            GrassVariant::Grass19 => "grass19",
            GrassVariant::Grass20 => "grass20",
        }
    }

    fn color(&self) -> Option<Color> {
        Some(if self.dead {
            Colors::DEAD_PLANT
        } else {
            Colors::PLANT
        })
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
    #[serde(rename = "15")]
    Grass15,
    #[serde(rename = "16")]
    Grass16,
    #[serde(rename = "17")]
    Grass17,
    #[serde(rename = "18")]
    Grass18,
    #[serde(rename = "19")]
    Grass19,
    #[serde(rename = "20")]
    Grass20,
}

impl Distribution<GrassVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GrassVariant {
        match rng.gen_range(0..20) {
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
            14 => GrassVariant::Grass15,
            15 => GrassVariant::Grass16,
            16 => GrassVariant::Grass17,
            17 => GrassVariant::Grass18,
            18 => GrassVariant::Grass19,
            19 => GrassVariant::Grass20,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Grass, GrassVariant, Terrain, TerrainInteract, TerrainView};

    #[test]
    fn test_dead_grass() {
        let mut terrain: Terrain = Grass::new(GrassVariant::Grass1, false).into();
        assert_eq!("grass", terrain.name());
        assert!(terrain.is_transparent());
        assert!(terrain.is_diggable());
        if let Terrain::Grass(grass) = &mut terrain {
            grass.die();
        } else {
            unreachable!()
        }
        assert_eq!("dead grass", terrain.name());
    }
}
