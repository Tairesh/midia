use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::game::Item;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Named {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Colored {
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Readable {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LookLike {
    pub look_like: String,
}

// TODO: slots and stuff
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wearable {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Container {
    pub items: Vec<Item>,
}
