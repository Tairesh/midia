use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::races::BodySlot;
use crate::game::savage::Damage;

// TODO: do we really need tags?
#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemTag {
    Tool,
    Weapon,
    Book,
    Corpse,
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
pub enum WearLayer {
    Inner,
    Middle,
    Outer,
    Clipped,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeleeDamageValue {
    pub damage: Damage,
    #[serde(default)]
    pub distance: u8,
    #[serde(default)]
    pub penetration: u8,
    // TODO: fighting modifier
    // TODO: minumum strength
    // TODO: parry modifier
}

impl Default for MeleeDamageValue {
    /// Attack with fists
    fn default() -> Self {
        Self {
            damage: Damage::default(),
            distance: 0,
            penetration: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WearableValue {
    pub layer: WearLayer,
    pub armor: u8,
    pub variants: Vec<HashSet<BodySlot>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPrototype {
    pub id: String,
    pub name: String,
    pub looks_like: String,
    // in grams
    pub mass: u32,
    #[serde(default)]
    pub tags: HashSet<ItemTag>,
    #[serde(default)]
    pub qualities: HashSet<ItemQuality>,
    #[serde(default)]
    pub two_handed_tool: bool,
    #[serde(default)]
    pub wearable: Option<WearableValue>,
    #[serde(default)]
    pub melee_damage: Option<MeleeDamageValue>,
}
