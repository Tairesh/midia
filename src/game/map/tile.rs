#![allow(dead_code)]

use std::collections::HashSet;

use super::{
    items::Item,
    terrain::{Terrain, TerrainInteract, TerrainView},
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
        if let Some(new_terrain) = self.terrain.on_step() {
            self.terrain = new_terrain;
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

    pub fn is_readable(&self) -> bool {
        self.terrain.is_readable() || self.items.iter().any(Item::is_readable)
    }

    pub fn read(&self) -> String {
        // TODO: probably we shouldn't read only first occurrence
        if self.terrain.is_readable() {
            return format!(
                "Text on this {} says «{}»",
                self.terrain.name(),
                self.terrain.read()
            );
        }

        self.items
            .iter()
            .rev()
            .filter(|i| i.is_readable())
            .map(|i| {
                if let Some(text) = i.read() {
                    if i.is_book() {
                        format!("This {} is called «{text}»", i.name())
                    } else {
                        format!("Text on this {} says «{text}»", i.name())
                    }
                } else if i.is_book() {
                    format!("This {} has no title on the cover", i.name())
                } else {
                    format!("This {} is unreadable", i.name())
                }
            })
            .next()
            .unwrap_or_else(|| "You can't find anything to read here.".to_string())
    }
}
