use crate::game::GameData;

use super::Item;

pub fn cloak() -> Item {
    let game_data = GameData::instance();
    Item::new(game_data.items.get("cloak").cloned().unwrap())
}

pub fn hat() -> Item {
    let game_data = GameData::instance();
    Item::new(game_data.items.get("hat").cloned().unwrap())
}

pub fn axe() -> Item {
    let game_data = GameData::instance();
    Item::new(game_data.items.get("axe").cloned().unwrap())
}

pub fn shovel() -> Item {
    let game_data = GameData::instance();
    Item::new(game_data.items.get("shovel").cloned().unwrap())
}
