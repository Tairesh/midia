use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::races::BodySlot;
use crate::game::savage::Damage;
use crate::game::Item;

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
pub struct MeleeDamageValue {
    pub moves: u8,
    pub damage: Damage,
    #[serde(default)]
    pub distance: u8,
    #[serde(default)]
    pub penetration: u8,
    // TODO: fighting modifier
    // TODO: minumum strength
    // TODO: defense modifier
}

impl MeleeDamageValue {
    /// Attack with item
    pub fn item(item: &Item) -> Self {
        Self {
            moves: item.attack_time().round() as u8,
            damage: Damage::default(),
            distance: 0,
            penetration: 0,
        }
    }
}

impl Default for MeleeDamageValue {
    /// Attack with fists
    fn default() -> Self {
        Self {
            moves: 10,
            damage: Damage::default(),
            distance: 0,
            penetration: 0,
        }
    }
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
    #[serde(default)]
    pub melee_damage: Option<MeleeDamageValue>,
}
