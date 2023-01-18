use serde::{Deserialize, Serialize};

use crate::game::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub items: Vec<Item>,
}
