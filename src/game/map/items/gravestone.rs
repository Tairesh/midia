use super::super::{
    item::{ItemInteract, ItemView},
    terrains::GraveData,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Gravestone {
    pub data: GraveData,
}

impl Gravestone {
    pub fn new(data: GraveData) -> Self {
        Self { data }
    }
}

impl ItemView for Gravestone {
    fn name(&self) -> String {
        "gravestone".to_string()
    }

    fn looks_like(&self) -> &'static str {
        "grave_stone"
    }
}

impl ItemInteract for Gravestone {
    fn mass(&self) -> u32 {
        200_000
    }

    fn is_readable(&self) -> bool {
        true
    }

    fn read(&self) -> String {
        self.data.read()
    }
}
