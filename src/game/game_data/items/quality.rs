use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::AmmoType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemQuality {
    /// can be used for digging
    Dig,
    /// can be used for butchering corpses
    Butch,
    /// can be used for cutting things
    Cut,
    /// can be used for chopping trees
    Chop,
    /// can store items
    Container {
        volume: u8,
        #[serde(default)]
        for_ammo: HashSet<AmmoType>,
    },
}
