use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::races::BodySlot;

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemTag {
    Tool,
    Weapon,
    Book,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemQuality {
    Dig,
    Butch,
    Cut,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemSpecial {
    Named,
    LookLike,
    Mass,
    Readable,
    Colored,
    Container,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WearLayer {
    Inner,
    Middle,
    Outer,
    Clipped,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPrototype {
    pub id: String,
    pub name: String,
    pub look_like: String,
    // in grams
    pub mass: u32,
    #[serde(default)]
    pub tags: HashSet<ItemTag>,
    #[serde(default)]
    pub qualities: HashSet<ItemQuality>,
    #[serde(default)]
    pub specials: HashSet<ItemSpecial>,
    #[serde(default)]
    pub two_handed_tool: bool,
    #[serde(default)]
    pub wearable: Option<HashSet<(BodySlot, WearLayer, u8)>>,
}
