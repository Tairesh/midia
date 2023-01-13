#![allow(dead_code)]

use std::collections::HashSet;

use rand::Rng;

use super::{
    items::Item,
    terrain::{Terrain, TerrainInteract},
    terrains::{Dirt, DirtVariant},
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tile {
    #[serde(rename = "t")]
    pub terrain: Terrain,
    #[serde(rename = "i")]
    pub items: Vec<Item>,
    #[serde(default)]
    #[serde(rename = "u")]
    pub units: HashSet<usize>,
}

impl Tile {
    pub fn new(terrain: Terrain) -> Self {
        Self {
            terrain,
            items: Vec::new(),
            units: HashSet::new(),
        }
    }

    /// Calls when avatar leaves tile
    pub fn off_step(&mut self, unit_id: usize) {
        self.units.remove(&unit_id);
    }

    /// Calls when avatar walks on tile
    pub fn on_step(&mut self, unit_id: usize) {
        self.units.insert(unit_id);
        // TODO: (for future) footprints
        if rand::thread_rng().gen_bool(0.1) {
            match self.terrain {
                Terrain::Grass(..) => {
                    self.terrain = Dirt::new(rand::random::<DirtVariant>()).into();
                }
                Terrain::Dirt(..) => {
                    self.terrain = Dirt::new(DirtVariant::Flat).into();
                }
                _ => {}
            }
        }
    }

    pub fn kill_grass(&mut self) {
        if let Terrain::Grass(grass) = &mut self.terrain {
            grass.die();
        }
    }

    pub fn top_item(&self) -> Option<&Item> {
        self.items.last()
    }

    pub fn dig(&mut self) -> Vec<Item> {
        let (terrain, items) = self.terrain.dig_result();
        self.terrain = terrain;
        items
    }

    pub fn is_readable(&self) -> bool {
        if self.terrain.is_readable() {
            return true;
        }

        self.items.iter().any(Item::is_readable)
    }

    pub fn read(&self) -> String {
        // TODO: probably we shouldn't read only first occurrence
        if self.terrain.is_readable() {
            return self.terrain.read();
        }

        self.items
            .iter()
            .rev()
            .filter(|i| i.is_readable())
            .map(|i| i.read().unwrap().to_string())
            .next()
            .unwrap_or_else(|| "You can't find anything to read here.".to_string())
    }
}
