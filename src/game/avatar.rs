#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::{
    map::items::{Cloak, Hat},
    races::Personality,
    savage::Attributes,
    Action, Item,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub personality: Personality,
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    // TODO: custom struct with hands counter and methods to return names and icons for UI
    pub wield: Vec<Item>,
    // TODO: custom struct with layers for dress and methods to return names and icons for UI
    pub wear: Vec<Item>,
    pub attributes: Attributes,
    // TODO: stamina
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn new(personality: Personality, attributes: Attributes, pos: Point) -> Self {
        Avatar {
            personality,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            wear: Vec::new(),
            attributes,
        }
    }

    // TODO: remove this and select dress in create character scene
    pub fn dressed_default(personality: Personality, attributes: Attributes, pos: Point) -> Self {
        Self {
            wear: vec![Hat::new().into(), Cloak::new().into()],
            ..Self::new(personality, attributes, pos)
        }
    }

    pub fn name_for_actions(&self) -> String {
        if self.personality.is_player {
            "You".to_string()
        } else {
            self.personality.mind.name.clone()
        }
    }
}
