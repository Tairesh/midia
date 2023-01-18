use std::collections::HashSet;

use crate::colors::Colors;
use crate::game::{Avatar, GameData, ItemPrototype, ItemTag};

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

pub fn dead_body(unit: &Avatar) -> Item {
    Item::new(ItemPrototype {
        id: "corpse".to_string(),
        name: format!("dead {}", unit.personality.age_name()),
        look_like: "corpse".to_string(),
        mass: unit.personality.appearance.body_mass(),
        tags: HashSet::from([ItemTag::Corpse]),
        qualities: HashSet::default(),
        specials: HashSet::default(),
        two_handed_tool: false,
        wearable: None,
        melee_damage: None,
    })
}
