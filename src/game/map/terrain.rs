use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::assets::Sprite;

use super::{
    terrains::{Boulder, Chest, Dirt, Grass, Tree},
    Item, Passage,
};

// TODO: JSON-ize all terrains

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Terrain {
    Dirt,
    Grass,
    Boulder,
    Tree,
    Chest,
}

#[enum_dispatch(Terrain)]
pub trait TerrainView {
    // TODO: implement Name and LooksLike after JSON-izing all terrains
    fn name(&self) -> &str;
    fn looks_like(&self) -> Sprite;
    fn color(&self) -> Option<Color> {
        None
    }
    fn is_transparent(&self) -> bool; // for FOV
}

#[enum_dispatch(Terrain)]
pub trait TerrainInteract {
    fn passage(&self) -> Passage;
    fn is_passable(&self) -> bool {
        matches!(self.passage(), Passage::Passable(..))
    }
    fn read(&self) -> String {
        unimplemented!()
    }
    fn can_contain_items(&self) -> bool {
        false
    }
    fn open(&self) -> (Terrain, Vec<Item>) {
        unimplemented!()
    }
    fn close(&self, _items: Vec<Item>) -> Terrain {
        unimplemented!()
    }
    fn on_step(&self) -> Option<Terrain> {
        None
    }
    fn smash(&self) -> Option<TerrainSmash> {
        None
    }
    fn supports_action(&self, _action: TerrainInteractAction) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TerrainInteractAction {
    Open,
    Close,
    Read,
    Drop,
    Examine,
    WieldFromGround,
}

pub struct TerrainSmash {
    pub toughness: u8,
    pub result: (Terrain, Vec<Item>),
}

impl TerrainSmash {
    pub fn new(toughness: u8, result: (Terrain, Vec<Item>)) -> Self {
        Self { toughness, result }
    }
}
