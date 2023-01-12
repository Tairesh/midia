use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{
    terrains::{Boulder, Dirt, Grass, Pit, Tree},
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
    Pit,
    Tree,
}

#[enum_dispatch(Terrain)]
pub trait TerrainView {
    fn name(&self) -> &str;
    // TODO: probably use String
    fn looks_like(&self) -> &'static str;
    fn is_transparent(&self) -> bool; // for FOV
}

#[enum_dispatch(Terrain)]
pub trait TerrainInteract {
    // TODO: implement Interact enum for adding more interaction types easily
    // see https://github.com/Tairesh/SpaceAge/blob/e90817c4f8ef25eee1d6fdc1986dd910dcdfbec6/src/game/ship_parts/mod.rs#L51
    fn passage(&self) -> Passage;
    fn is_passable(&self) -> bool {
        matches!(self.passage(), Passage::Passable(..))
    }
    fn is_diggable(&self) -> bool {
        false
    }
    /// return new Terrain and digged items
    fn dig_result(&self) -> (Terrain, Vec<Item>) {
        unimplemented!()
    }
    fn is_readable(&self) -> bool {
        false
    }
    fn read(&self) -> String {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::terrains::{Dirt, DirtVariant, Grass, GrassVariant},
        Terrain, TerrainInteract, TerrainView,
    };

    #[test]
    fn test_dirt() {
        let terrain: Terrain = Dirt::new(DirtVariant::Flat).into();
        assert_eq!("flat dirt", terrain.name());
        assert!(terrain.is_diggable());
    }

    #[test]
    fn test_dead_grass() {
        let mut terrain: Terrain = Grass::new(GrassVariant::Grass9).into();
        assert_eq!("grass", terrain.name());
        if let Terrain::Grass(grass) = &mut terrain {
            grass.die();
        } else {
            unreachable!()
        }
        assert_eq!("dead grass", terrain.name());
    }
}
