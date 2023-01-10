use super::super::item::{ItemInteract, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Cloak {}

impl Cloak {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Cloak {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemView for Cloak {
    fn name(&self) -> String {
        "cloak".to_string()
    }

    fn looks_like(&self) -> &'static str {
        "cloak"
    }
}

impl ItemInteract for Cloak {
    fn mass(&self) -> u32 {
        300
    }

    fn is_wearable(&self) -> bool {
        true
    }
}
