use super::super::item::{ItemInteract, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Rags {}

impl Rags {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Rags {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemView for Rags {
    fn name(&self) -> String {
        "dirty rags".to_string()
    }

    fn looks_like(&self) -> &'static str {
        "cloak"
    }
}

impl ItemInteract for Rags {
    fn mass(&self) -> u32 {
        300
    }

    fn is_wearable(&self) -> bool {
        true
    }
}
