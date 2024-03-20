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
    fn can_suck_items_on_close(&self) -> bool {
        false
    }
    fn open(&self) -> (Terrain, Vec<Item>) {
        unimplemented!()
    }
    fn close(&self) -> Terrain {
        unimplemented!()
    }
    fn close_and_suck_items(&self, _items: Vec<Item>) -> Terrain {
        unimplemented!()
    }
    fn on_step(&self) -> Option<Terrain> {
        None
    }
    fn is_smashable(&self) -> bool {
        false
    }
    fn smash_toughness(&self) -> u8 {
        unimplemented!()
    }
    fn smash_result(&self) -> (Terrain, Vec<Item>) {
        unimplemented!()
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
