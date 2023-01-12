#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::{
    map::items::{Cloak, Hat},
    races::Personality,
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
    pub wear: Vec<Item>,
    pub stamina: u8,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn new(personality: Personality, pos: Point) -> Self {
        Avatar {
            personality,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            wear: Vec::new(),
            stamina: 100,
        }
    }

    pub fn dressed_default(personality: Personality, pos: Point) -> Self {
        Self {
            wear: vec![Hat::new().into(), Cloak::new().into()],
            ..Self::new(personality, pos)
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
