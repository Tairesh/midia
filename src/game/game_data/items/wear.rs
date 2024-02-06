use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::BodySlot;

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WearLayer {
    Inner,
    Middle,
    Outer,
    Clipped,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WearableValue {
    pub layer: WearLayer,
    pub armor: u8,
    pub variants: Vec<HashSet<BodySlot>>,
}
