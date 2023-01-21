use std::collections::HashSet;

use crate::colors::Colors;
use crate::game::{Avatar, ItemPrototype, ItemSize, ItemTag};

use super::Item;

pub fn random_book() -> Item {
    Item::new("book")
        .with_colored(Colors::BLUE_VIOLET)
        .with_named("strange book")
        .with_readable("Lore of the Midia")
}

pub fn dead_body(unit: &Avatar) -> Item {
    Item::custom(ItemPrototype {
        id: "corpse".to_string(),
        name: format!("dead {}", unit.personality.age_name()),
        looks_like: "corpse".to_string(),
        size: ItemSize::Huge,
        tags: HashSet::from([ItemTag::Corpse]),
        qualities: HashSet::default(),
        two_handed_tool: false,
        wearable: None,
        melee_damage: None,
    })
}
