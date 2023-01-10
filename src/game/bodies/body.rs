use std::collections::HashMap;

use geometry::Point;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::super::{map::items::BodyPart, Item};

pub type BodyPartsCollections = HashMap<Point, BodyPart>;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    #[serde(rename = "p")]
    #[serde_as(as = "Vec<(_, _)>")]
    pub parts: BodyPartsCollections,
    #[serde(rename = "w")]
    pub wear: Vec<Item>,
}

impl Body {
    pub fn new(parts: BodyPartsCollections) -> Self {
        Self {
            parts,
            wear: Vec::default(),
        }
    }
}
