use crate::colors::Colors;
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

pub fn random_book() -> Item {
    let game_data = GameData::instance();
    Item::new(game_data.items.get("book").cloned().unwrap())
        .with_colored(Colors::BLUE_VIOLET)
        .with_named("strange book")
        .with_readable("Lore of the Midia")
}

pub fn backpack() -> Item {
    let game_data = GameData::instance();
    Item::new(game_data.items.get("backpack").cloned().unwrap()).with_container(Vec::new())
}
