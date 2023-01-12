use std::collections::HashSet;

use super::super::item::{ItemInteract, ItemQuality, ItemTag, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Knife {}

impl Knife {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Knife {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemView for Knife {
    fn name(&self) -> String {
        "knife".to_string()
    }

    fn looks_like(&self) -> &'static str {
        "knife"
    }
}

impl ItemInteract for Knife {
    fn tags(&self) -> HashSet<ItemTag> {
        HashSet::from([ItemTag::Tool, ItemTag::Weapon])
    }

    fn qualities(&self) -> HashSet<ItemQuality> {
        HashSet::from([ItemQuality::Butch])
    }

    fn mass(&self) -> u32 {
        500
    }
}
