use super::super::item::{ItemInteract, ItemView};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Hat {}

impl Hat {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Hat {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemView for Hat {
    fn name(&self) -> String {
        "hat".to_string()
    }

    fn looks_like(&self) -> &'static str {
        "hat"
    }
}

impl ItemInteract for Hat {
    fn mass(&self) -> u32 {
        100
    }

    fn is_wearable(&self) -> bool {
        true
    }
}
