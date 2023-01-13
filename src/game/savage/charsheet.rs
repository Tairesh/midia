use super::{Attributes, Skills};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct CharSheet {
    pub attributes: Attributes,
    pub skills: Skills,
}

impl CharSheet {
    pub fn random() -> Self {
        Self {
            attributes: Attributes::random(),
            skills: Skills::default(),
        }
    }
}
