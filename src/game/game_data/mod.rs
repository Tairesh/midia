use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;

use data_entity::DataEntity;
pub use items::{
    AmmoType, DamageValue, IsAmmoValue, ItemPrototype, ItemQuality, ItemSize, Material,
    NeedAmmoValue, WearLayer,
};

use crate::game::races::{Race, Sex};

mod data_entity;
mod items;
mod names_pack;
pub mod pregen;

const PATH: &str = "data";
static INSTANCE: OnceCell<GameData> = OnceCell::new();

#[derive(Debug)]
pub struct GameData {
    pub names: HashMap<Race, HashMap<Sex, Vec<String>>>,
    pub items: HashMap<String, ItemPrototype>,
}

impl GameData {
    pub fn instance() -> &'static Self {
        INSTANCE.get_or_init(Self::load)
    }

    pub fn get_item_prototype(&self, id: &str) -> &ItemPrototype {
        if let Some(proto) = self.items.get(id) {
            proto
        } else {
            panic!("No item prototype with id: {id}");
        }
    }

    fn load() -> Self {
        let mut data = Self {
            names: Race::iterator()
                .map(|r| (r, Sex::iterator().map(|s| (s, Vec::new())).collect()))
                .collect(),
            items: HashMap::with_capacity(100),
        };

        data.load_dir(&PathBuf::from(PATH));

        data
    }

    fn load_dir(&mut self, path: &Path) {
        for entry in path.read_dir().unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                self.load_dir(&path);
            } else if let Some(ext) = path.extension() {
                if ext == "json" {
                    self.load_file(&path);
                }
            }
        }
    }

    fn load_file(&mut self, path: &Path) {
        if let Ok(file) = File::open(path) {
            let result = serde_json::from_reader::<_, Vec<DataEntity>>(BufReader::new(file));
            if let Ok(entities) = result {
                for entity in entities {
                    self.add_entity(entity);
                }
            } else {
                // TODO: implement logging
                println!("Failed to load file: {path:?}, {result:?}");
            }
        }
    }

    fn add_entity(&mut self, entity: DataEntity) {
        match entity {
            DataEntity::Item(item) => {
                self.items.insert(item.id.clone(), item);
            }
            DataEntity::NamesPack(name_pack) => {
                for (race, value) in name_pack.names {
                    for (sex, names) in value {
                        self.names
                            .get_mut(&race)
                            .unwrap()
                            .get_mut(&sex)
                            .unwrap()
                            .extend(names);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::races::{Race, Sex};

    use super::GameData;

    #[test]
    fn data_load() {
        let data = GameData::load();
        assert!(
            data.names
                .get(&Race::Gazan)
                .unwrap()
                .get(&Sex::Male)
                .unwrap()
                .len()
                > 0
        );
        assert!(data
            .names
            .get(&Race::Gazan)
            .unwrap()
            .get(&Sex::Male)
            .unwrap()
            .contains(&"Dragan".to_string()));
        assert!(data.items.len() > 0);
        assert!(data.items.contains_key("book"));
    }
}
